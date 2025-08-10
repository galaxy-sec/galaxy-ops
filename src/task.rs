use derive_getters::Getters;
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::{error::MainResult, module::ModelSTD};

#[derive(Clone, Debug, PartialEq, Display, Serialize, Deserialize)]
pub enum OperationType {
    #[display("setup")]
    Setup,
    #[display("update")]
    Update,
    #[display("port")]
    Port,
    #[display("backup")]
    Backup,
    #[display("clean")]
    Clean,
    #[display("uninstall")]
    UnInstall,
    Other,
}
pub trait Task {
    fn exec(&self) -> MainResult<()>;
}

pub type TaskHandle = Box<dyn Task>;

pub trait NodeSetupTaskBuilder {
    fn make_setup_task(&self, node: &ModelSTD) -> MainResult<TaskHandle>;
}

pub trait UpdateTaskMaker {
    fn make_update_task(&self) -> MainResult<TaskHandle>;
}

#[derive(Getters)]
pub struct CombinedTask {
    name: String,
    subs: Vec<TaskHandle>,
}
impl CombinedTask {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            subs: Vec::new(),
        }
    }
    pub fn add_sub(&mut self, sub: TaskHandle) {
        self.subs.push(sub);
    }
}
impl Task for CombinedTask {
    fn exec(&self) -> MainResult<()> {
        for task in &self.subs {
            task.exec()?;
        }
        Ok(())
    }
}

pub struct EchoTask {
    cmd: String,
}
impl EchoTask {
    pub fn new<S: Into<String>>(cmd: S) -> Self {
        Self { cmd: cmd.into() }
    }
}

impl Task for EchoTask {
    fn exec(&self) -> MainResult<()> {
        println!("echo task:\n{}\n", self.cmd);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::ElementReason, error::MainReason, module::ModelSTD};

    #[test]
    fn test_operation_type_display() {
        assert_eq!(format!("{}", OperationType::Setup), "setup");
        assert_eq!(format!("{}", OperationType::Update), "update");
        assert_eq!(format!("{}", OperationType::Port), "port");
        assert_eq!(format!("{}", OperationType::Backup), "backup");
        assert_eq!(format!("{}", OperationType::Clean), "clean");
        assert_eq!(format!("{}", OperationType::UnInstall), "uninstall");
        assert_eq!(format!("{}", OperationType::Other), "Other");
    }

    #[test]
    fn test_operation_type_debug() {
        let debug_str = format!("{:?}", OperationType::Setup);
        assert!(debug_str.contains("Setup"));
    }

    #[test]
    fn test_operation_type_clone() {
        let op = OperationType::Setup;
        let cloned = op.clone();
        assert_eq!(op, cloned);
    }

    #[test]
    fn test_operation_type_partial_eq() {
        assert_eq!(OperationType::Setup, OperationType::Setup);
        assert_ne!(OperationType::Setup, OperationType::Update);
    }

    #[test]
    fn test_echo_task_new() {
        let task = EchoTask::new("test command");
        assert_eq!(task.cmd, "test command");
    }

    #[test]
    fn test_echo_task_exec() {
        let task = EchoTask::new("echo hello");
        let result = task.exec();
        assert!(result.is_ok());
    }

    #[test]
    fn test_echo_task_exec_content() {
        let task = EchoTask::new("test message");
        let result = task.exec();
        assert!(result.is_ok());
        // Note: We can't easily capture println! output in tests
        // But we can verify it doesn't panic
    }

    #[test]
    fn test_combined_task_new() {
        let task = CombinedTask::new("test task");
        assert_eq!(task.name(), "test task");
        assert!(task.subs().is_empty());
    }

    #[test]
    fn test_combined_task_add_sub() {
        let mut task = CombinedTask::new("combined");
        let echo_task = Box::new(EchoTask::new("sub task"));

        task.add_sub(echo_task);
        assert_eq!(task.subs().len(), 1);
    }

    #[test]
    fn test_combined_task_exec_empty() {
        let task = CombinedTask::new("empty");
        let result = task.exec();
        assert!(result.is_ok());
    }

    #[test]
    fn test_combined_task_exec_with_subs() {
        let mut task = CombinedTask::new("combined");
        let sub1 = Box::new(EchoTask::new("sub1"));
        let sub2 = Box::new(EchoTask::new("sub2"));

        task.add_sub(sub1);
        task.add_sub(sub2);

        let result = task.exec();
        assert!(result.is_ok());
    }

    #[test]
    fn test_combined_task_exec_with_failing_sub() {
        let mut task = CombinedTask::new("combined");

        // Create a mock failing task
        struct FailingTask;
        impl Task for FailingTask {
            fn exec(&self) -> MainResult<()> {
                Err(MainReason::Element(ElementReason::Miss("Task failed".to_string())).into())
            }
        }

        task.add_sub(Box::new(FailingTask));
        let result = task.exec();
        assert!(result.is_err());
    }

    // Mock test for NodeSetupTaskBuilder trait
    #[test]
    fn test_node_setup_task_builder() {
        struct MockBuilder;
        impl NodeSetupTaskBuilder for MockBuilder {
            fn make_setup_task(&self, _node: &ModelSTD) -> MainResult<TaskHandle> {
                Ok(Box::new(EchoTask::new("mock setup")))
            }
        }

        let builder = MockBuilder;
        let mock_model = ModelSTD::x86_ubt22_host(); // Using existing constructor

        let result = builder.make_setup_task(&mock_model);
        assert!(result.is_ok());
    }

    // Mock test for UpdateTaskMaker trait
    #[test]
    fn test_update_task_maker() {
        struct MockUpdater;
        impl UpdateTaskMaker for MockUpdater {
            fn make_update_task(&self) -> MainResult<TaskHandle> {
                Ok(Box::new(EchoTask::new("mock update")))
            }
        }

        let updater = MockUpdater;
        let result = updater.make_update_task();
        assert!(result.is_ok());
    }

    #[test]
    fn test_task_trait_object_safety() {
        // Test that Task can be used as a trait object
        let task: Box<dyn Task> = Box::new(EchoTask::new("test"));
        let result = task.exec();
        assert!(result.is_ok());
    }

    #[test]
    fn test_operation_type_serialization() {
        let op = OperationType::Setup;
        let serialized = serde_json::to_string(&op).unwrap();
        assert_eq!(serialized, "\"setup\"");
    }

    #[test]
    fn test_operation_type_deserialization() {
        let deserialized: OperationType = serde_json::from_str("\"update\"").unwrap();
        assert_eq!(deserialized, OperationType::Update);
    }
}
