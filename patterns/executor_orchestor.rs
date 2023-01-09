use std::sync::{Arc, Mutex};
use std::thread;

// A struct that holds the state of the execution
struct ExecutionState {
    running: bool,
    result: Option<i32>,
}

impl ExecutionState {
    fn new() -> Self {
        Self {
            running: false,
            result: None,
        }
    }
}

// A trait that defines the behavior of a task that can be executed
trait Executable {
    fn execute(&mut self) -> i32;
}

// An implementation of the Executable trait
struct Task {
    data: i32,
}

impl Executable for Task {
    fn execute(&mut self) -> i32 {
        self.data * 2
    }
}

// The orchestrator struct that manages the execution of tasks
struct ExecutionOrchestrator {
    state: Arc<Mutex<ExecutionState>>,
}

impl ExecutionOrchestrator {
    fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ExecutionState::new())),
        }
    }

    // Start the execution of a task
    fn start<T>(&self, task: T)
    where
        T: Executable + Send + 'static,
    {
        let state = self.state.clone();
        let handle = thread::spawn(move || {
            // Lock the state and update it to running
            let mut state = state.lock().unwrap();
            state.running = true;

            // Execute the task and store the result in the state
            state.result = Some(task.execute());
            state.running = false;
        });

        handle.join().unwrap();
    }

    // Check the status of the execution
    fn check_status(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.running
    }

    // Get the result of the execution
    fn get_result(&self) -> Option<i32> {
        let state = self.state.lock().unwrap();
        state.result
    }
}

fn main() {
    let orchestrator = ExecutionOrchestrator::new();
    let task = Task { data: 42 };

    // Start the execution of the task
    orchestrator.start(task);

    // Wait until the execution is complete
    while orchestrator.check_status() {}

    // Get the result of the execution
    let result = orchestrator.get_result().unwrap();
    println!("Result: {}", result);
}
