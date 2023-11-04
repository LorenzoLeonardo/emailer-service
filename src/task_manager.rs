use tokio::sync::mpsc::UnboundedReceiver;

pub enum TaskMessage {}

pub struct TaskManager {
    rx: UnboundedReceiver<TaskMessage>,
}

impl TaskManager {
    pub fn new(rx: UnboundedReceiver<TaskMessage>) -> Self {
        Self { rx }
    }
    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                    Some(_msg) = self.rx.recv() => {
                }
            }
        }
    }
}
