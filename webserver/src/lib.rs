use std::{usize, thread, sync::{mpsc, Arc, Mutex}};


//首先 Worker 结构体需要从线程池 TreadPool 的队列中获取待执行的代码
//对于这类场景，消息传递非常适合：我们将使用消息通道( channel )作为任务队列。

pub struct ThreadPool{
    thread : Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce()+Send+'static>;

impl ThreadPool {
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        //将传过来func 封装成Job类型
        let job = Box::new(f);
        //作为消息队列生产者，发送消息
        self.sender.send(job).unwrap();
    }
    //ThreadPool constructed func
    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0);
        //定义一个消息队列
        let (sender,receiver) = mpsc::channel();
        //
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(size);
        for i in 0..size{
            threads.push(Worker::new(i,Arc::clone(&receiver)));
        }
        ThreadPool { thread: threads ,sender:sender}
    }
}

///作为线程池和任务线程间的桥梁，任务在·是获得将要执行的代码，然后在具体的线程中去执行
// 消息通道有发送端和接收端，其中线程池 ThreadPool 持有发送端，通过 execute 方法来发送任务
//接收端是Worker，它的内部线程将接收任务，然后进行处理。
struct Worker{
    id : usize,
    thread: thread::JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>>
}

impl Worker {
    //Worker constructed func
    fn new(id: usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });
        Worker { id: id, thread: thread }
    }
}