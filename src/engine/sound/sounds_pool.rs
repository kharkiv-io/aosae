use crate::engine::core::rmv::{self, ResourceManager};
use crate::engine::io::decoder::METADATA;
use crate::engine::io::output::{self, create_samples_box, play_samples_graph};
use crate::engine::sound::rails;
use anyhow::Result;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use symphonia::core::meta::StandardTagKey;
#[derive(Debug)]
pub struct SoundTask {
    id: usize,
    samples: Arc<Vec<f32>>,
    duration: Duration,
    delay: Duration,
    scheduled_time: Instant,
}
#[derive(Debug)]
pub struct SoundThreadPool {
    flowers: Vec<Flower>,
    sender: Option<std::sync::mpsc::Sender<Message>>,
    task_queue: Arc<Mutex<VecDeque<SoundTask>>>,
    active_tasks: Arc<Mutex<usize>>,
    start_time: Instant,
}

type JobFor = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(JobFor),
    Terminate,
}
#[derive(Debug)]
struct Flower {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl SoundThreadPool {
    pub fn new(size: usize) -> Result<SoundThreadPool> {
        if size == 0 {
            return Err(anyhow::anyhow!(
                "Idiot, size of pool must be greater than zero!"
            ));
        }
        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let task_queue = Arc::new(Mutex::new(VecDeque::new()));
        let active_tasks = Arc::new(Mutex::new(0));
        let mut flowers = Vec::with_capacity(size);
        let start_time = Instant::now();
        for id in 0..size {
            flowers.push(Flower::new(
                id,
                Arc::clone(&receiver),
                Arc::clone(&task_queue),
                Arc::clone(&active_tasks),
                start_time,
            ));
        }
        Ok(SoundThreadPool {
            flowers,
            sender: Some(sender),
            task_queue,
            active_tasks,
            start_time,
        })
    }

    pub fn execute(
        &self,
        id: usize,
        samples: Vec<f32>,
        duration: Duration,
        delay: Duration,
    ) -> Result<()> {
        let scheduled_time = self.start_time + delay;
        let task = SoundTask {
            id,
            samples: Arc::new(samples),
            duration,
            delay,
            scheduled_time,
        };
        let mut queue = self
            .task_queue
            .lock()
            .map_err(|e| anyhow::anyhow!("Failure while locking task queue -> {}", e))?;
        queue.push_back(task);
        let sender = self
            .sender
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Channel is not available?"))?;
        sender
            .send(Message::NewJob(Box::new(move || {})))
            .map_err(|e| anyhow::anyhow!("Failed to send JobFor -> {}", e))?;
        Ok(())
    }
    pub fn active_tasks(&self) -> Result<usize> {
        let count = self
            .active_tasks
            .lock()
            .map_err(|e| anyhow::anyhow!("Can't get active tasks count due to? ->  {}", e))?;
        Ok(*count)
    }
    pub fn wait_all(&self) -> Result<()> {
        loop {
            let count = self.active_tasks()?;
            if count == 0 {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }
}

impl Flower {
    fn new(
        id: usize,
        receiver: Arc<Mutex<std::sync::mpsc::Receiver<Message>>>,
        task_queue: Arc<Mutex<VecDeque<SoundTask>>>,
        active_tasks: Arc<Mutex<usize>>,
        start_time: Instant,
    ) -> Flower {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(_) => {
                        let task = {
                            let mut queue = task_queue.lock().unwrap();
                            queue.pop_front()
                        };

                        if let Some(task) = task {
                            let now = Instant::now();
                            if now < task.scheduled_time {
                                thread::sleep(task.scheduled_time - now);
                            }

                            {
                                let mut count = active_tasks.lock().unwrap();
                                *count += 1;
                            }
                            let samples = create_samples_box((*task.samples).to_owned());
                            play_samples_graph(samples, task.duration);
                            {
                                let mut count = active_tasks.lock().unwrap();
                                *count -= 1;
                            }
                        }
                    }
                    Message::Terminate => break,
                }
            }
        });

        Flower {
            id,
            thread: Some(thread),
        }
    }
}
impl Drop for SoundThreadPool {
    fn drop(&mut self) {
        if let Some(sender) = &self.sender {
            for _ in &self.flowers {
                sender.send(Message::Terminate).unwrap();
            }
        }
        for flower in &mut self.flowers {
            if let Some(thread) = flower.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
