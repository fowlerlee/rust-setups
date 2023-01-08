use std::error::Error;
use std::sync::Arc;

trait SagaStep {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
    fn compensate(&self) -> Result<(), Box<dyn Error>>;
}

struct Saga {
    steps: Vec<Box<dyn SagaStep>>,
}

impl Saga {
    fn new(steps: Vec<Box<dyn SagaStep>>) -> Self {
        Self { steps }
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        for step in &self.steps {
            step.execute()?;
        }
        Ok(())
    }

    fn compensate(&self) -> Result<(), Box<dyn Error>> {
        for step in self.steps.iter().rev() {
            step.compensate()?;
        }
        Ok(())
    }
}

struct Step1;

impl SagaStep for Step1 {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        // Execute the logic for this step
        Ok(())
    }

    fn compensate(&self) -> Result<(), Box<dyn Error>> {
        // Execute the compensation logic for this step
        Ok(())
    }
}

struct Step2;

impl SagaStep for Step2 {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        // Execute the logic for this step
        Ok(())
    }

    fn compensate(&self) -> Result<(), Box<dyn Error>> {
        // Execute the compensation logic for this step
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let saga = Saga::new(vec![
        Box::new(Step1),
        Box::new(Step2),
    ]);
    saga.run()?;
    Ok(())
}
