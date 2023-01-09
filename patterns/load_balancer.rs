use std::sync::{Arc, Mutex};
use std::thread;

// A struct that holds the state of a worker
struct WorkerState {
    busy: bool,
}

impl WorkerState {
    fn new() -> Self {
        Self { busy: false }
    }
}

// A trait that defines the behavior of a task that can be executed
trait Executable {
    fn execute(&self) -> i32;
}

// An implementation of the Executable trait
struct Task {
    data: i32,
}

impl Executable for Task {
    fn execute(&self) -> i32 {
        self.data * 2
    }
}

// A struct that represents a worker
struct Worker {
    id: usize,
    state: Arc<Mutex<WorkerState>>,
}

impl Worker {
    fn new(id: usize) -> Self {
        Self {
            id,
            state: Arc::new(Mutex::new(WorkerState::new())),
        }
    }

    // Execute a task
    fn execute<T>(&self, task: T)
    where
        T: Executable + Send + 'static,
    {
        // Lock the state and update it to busy
        let mut state = self.state.lock().unwrap();
        state.busy = true;

        let handle = thread::spawn(move{});
    }
}
