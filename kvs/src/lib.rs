use failure::Fail;
use serde::de::Unexpected::Str;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Error, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::Path;
use std::process::exit;
use std::string::String;
use std::{collections::HashMap, fs, io, path::PathBuf};

// 自定义错误
#[derive(Debug, Fail)]
pub enum KvError {
    /// Io error
    #[fail(display = "io error occurred.")]
    IoError(#[cause] io::Error),

    ///serde  error
    #[fail(display = "serde error occurred.")]
    SerdeErr(#[cause] serde_json::Error),

    #[fail(display = "Unexpect Command Type")]
    UnexpectedCommandType,
    #[fail(display = "Key Not Found")]
    KeyNotFound,
}

// 实现根据错误源响应对应的错误
impl From<serde_json::Error> for KvError {
    fn from(error: serde_json::Error) -> Self {
        KvError::SerdeErr(error)
    }
}

impl From<io::Error> for KvError {
    fn from(value: io::Error) -> Self {
        KvError::IoError(value)
    }
}

// 自定义Result类型，默认使用KvError作为错误类型
pub type Result<T> = std::result::Result<T, KvError>;

// 自定义 日志路径
const LOG_PATH: &str = "tmp.log";

// 定义枚举值 Commend，存放不同种类的命令
#[derive(Serialize, Deserialize, Debug)]
pub enum Commend {
    Set { key: String, value: String },
    Remove { key: String },
}
// 枚举结构体Commend 的构造函数
impl Commend {
    fn set(key: String, value: String) -> Commend {
        Commend::Set { key, value }
    }
    fn remove(key: String) -> Commend {
        Commend::Remove { key }
    }
}

#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    writer: BufWriterWithPos<File>,
    readers: HashMap<u64, BufReaderWithPos<File>>,
    index: BTreeMap<String, CommandPos>,
    current_gen: u64,
}

impl KvStore {
    ///set a key/value pair in the store
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        // 序列化set 命令
        let commend = Commend::set(key, value);
        //获取未插入数据前的pos位置
        let pos = self.writer.pos;
        //插入数据后，pos的位置会自动改变
        serde_json::to_writer(&mut self.writer, &commend)?;
        self.writer.flush()?;
        if let Commend::Set { key, .. } = commend {
            println!("要在索引数上构建日志位置了{}", key);

            self.index
                .insert(key, (self.current_gen, pos..self.writer.pos).into());
            // if let Some(old_cmd) = self
            //     .index
            //     .insert(key, (self.current_gen, pos..self.writer.pos).into())
            // {}
        }
        Ok(())
    }
    ///get a key/value pair from the store
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        //1、判断有没有key
        if !self.index.contains_key(&key) {
            return Ok(None);
        }
        // 2、根据CommendPos读取数据
        let commandPos = self.index.get(&key).unwrap();
        let mut reader = self.readers.get_mut(&commandPos.gen).unwrap();
        reader.seek(SeekFrom::Start(commandPos.pos))?;
        let command_reader = reader.take(commandPos.len);
        if let Commend::Set { value, .. } = serde_json::from_reader(command_reader)? {
            Ok(Some(value))
        } else {
            Err(KvError::UnexpectedCommandType)
        }
    }
    ///remove a key/value pair from the
    pub fn remove(&mut self, key: String) -> Result<()> {
        println!("remove key is {}", key);
        //判断键值索引是否包含该键
        if self.index.contains_key(&key) {
            //1、在日志中存入命令
            let rm_cmd = Commend::remove(key);
            let pos = self.writer.pos;
            serde_json::to_writer(&mut self.writer, &rm_cmd)?;
            self.writer.flush()?;
            //2、删除键值索引里面的值
            if let Commend::Remove { key } = rm_cmd {
                self.index.remove(&key).expect("kety not found");
            }
            Ok(())
        } else {
            Err(KvError::KeyNotFound)
        }
    }
    ///初始化KvStore
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        // 拿到路径
        let path = path.into();
        println!("日志文件路径是：{}", path.display());
        // 如果目录不存在，则级联创建目录
        fs::create_dir_all(&path)?;
        // 创建reader 和 index
        let mut readers: HashMap<u64, BufReaderWithPos<File>> = HashMap::new();
        let mut index: BTreeMap<String, CommandPos> = BTreeMap::new();
        // 获取数据文件夹下的所有日志文件的代号
        let gen_list = sorted_gen_list(&path)?;
        for &gen in &gen_list {
            let mut reader = BufReaderWithPos::new(File::open(log_path(&path, gen))?)?;
            //从日志文件中加载数据，然后构建内存中的键值索引
            load(gen, &mut reader, &mut index);
            readers.insert(gen, reader);
        }
        let current_gen = gen_list.last().unwrap_or(&0) + 1;
        let writer = new_log_file(&path, current_gen, &mut readers)?;
        Ok(KvStore {
            path,
            writer,
            readers,
            index,
            current_gen,
        })
    }
}

///返回指定文件夹下的文件名的u64，再经过排序；例如 1.log、2.log、3.log => 1，2，3
fn sorted_gen_list(path: &Path) -> Result<Vec<u64>> {
    let mut gen_list: Vec<u64> = fs::read_dir(path)?
        .flat_map(|res| -> Result<_> { Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();
    gen_list.sort_unstable();
    Ok(gen_list)
}

///返回文件处理后的文件路径
fn log_path(dir: &Path, gen: u64) -> PathBuf {
    dir.join(format!("{}.log", gen))
}
///返回日志文件的写入器
fn new_log_file(
    path: &Path,
    gen: u64,
    readers: &mut HashMap<u64, BufReaderWithPos<File>>,
) -> Result<BufWriterWithPos<File>> {
    let path = log_path(&path, gen);
    let writer = BufWriterWithPos::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?,
    )?;
    readers.insert(gen, BufReaderWithPos::new(File::open(&path)?)?);
    Ok(writer)
}

///  读取数据日志文件，重构键值索引，传入对应文件的读取器reader和全局的键值索引index
fn load(
    gen: u64,
    reader: &mut BufReaderWithPos<File>,
    index: &mut BTreeMap<String, CommandPos>,
) -> Result<u64> {
    // 1、设置从头读取数据
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    //2、从读取器中反序列数据量，并生成Command的迭代器
    let mut command_stream = serde_json::Deserializer::from_reader(reader).into_iter::<Commend>();
    while let Some(cmd) = command_stream.next() {
        //当前Command在日志中的末尾位置
        let new_pos = command_stream.byte_offset() as u64;
        match cmd? {
            Commend::Set { key, .. } => index.insert(key, (gen, pos..new_pos).into()),
            Commend::Remove { key } => index.remove(&key),
        };
        //更新下一个Command的开始位置
        pos = new_pos;
    }
    Ok(0 as u64)
}

///带有位置追踪功能的缓冲写入器
#[derive(Debug)]
struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    ///BufWriterWithPos对象的构造函数-关联函数
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}

///BufWriterWithPos 实现Write接口
impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

///缓冲写入器实现随机访问操作
impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}

///实现读缓冲器
#[derive(Debug)]
struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}
/// 读缓冲区的构造函数
impl<R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

///命令在日志中的位置
#[derive(Debug)]
struct CommandPos {
    gen: u64,
    pos: u64,
    len: u64,
}

impl From<(u64, Range<u64>)> for CommandPos {
    fn from((gen, range): (u64, Range<u64>)) -> Self {
        CommandPos {
            gen,
            pos: range.start,
            len: range.end - range.start,
        }
    }
}
