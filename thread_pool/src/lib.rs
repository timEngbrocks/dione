use std::{collections::HashMap, error::Error, sync::mpsc::{self, Receiver, Sender}, thread::{self, JoinHandle}};

#[derive(Default)]
pub struct ThreadPool {
    ids: Vec<usize>,
    handles: HashMap<usize, JoinHandle<()>>,
    tx: HashMap<usize, Sender<ControlMessage>>
}

impl ThreadPool {
    pub fn spawn<C, F>(&mut self, c: C, f: F)
    where
        C: Send + 'static,
        F: FnOnce(C, Receiver<ControlMessage>),
        F: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            f(c, rx)
        });

        let id = if self.ids.is_empty() {
            0
        } else {
            self.ids[self.ids.len() - 1] + 1
        };
        self.ids.push(id);
        self.handles.insert(id, handle);
        self.tx.insert(id, tx);
    }

    pub fn join_all(&mut self) -> Result<(), Box<dyn Error>> {
        for id in &self.ids {
            let tx = match self.tx.get(id) {
                Some(tx) => tx,
                None => {
                    return Err(format!("Thread id {id} not found in tx!"))?;
                }
            };
            if tx.send(ControlMessage::SHUTDOWN).is_err() {
                continue;
            }
            let handle = match self.handles.remove(id) {
                Some(handle) => handle,
                None => {
                    return Err(format!("Thread id {id} not found in handles!"))?;
                }
            };
            if handle.join().is_err() {
                return Err(format!("Thread id {id} failed to join!"))?;
            }
        }
        self.ids = Vec::new();
        self.handles = HashMap::new();
        self.tx = HashMap::new();
        Ok(())
    }
}

pub enum ControlMessage {
    SHUTDOWN
}