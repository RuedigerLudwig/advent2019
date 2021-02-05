#[derive(Debug, Copy, Clone)]
pub enum StepResult {
    Value(i64),
    Stop,
    Proceed,
    WaitForInput,
}
