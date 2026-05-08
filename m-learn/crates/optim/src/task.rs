use std::fmt;

pub type TaskResult<T> = Result<T, TaskError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskError {
    Panic(String),
    Cancelled,
}

impl std::error::Error for TaskError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::Panic(msg) => write!(f, "Task panicked: {}", msg),
            TaskError::Cancelled => write!(f, "Task was cancelled"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_error_display() {
        let err = TaskError::Panic("test panic".to_string());
        assert_eq!(format!("{}", err), "Task panicked: test panic");
    }

    #[test]
    fn test_task_error_equality() {
        let err1 = TaskError::Cancelled;
        let err2 = TaskError::Cancelled;
        assert_eq!(err1, err2);
    }
}
