use crate::{input::InputAction, load_all_pods, load_logs, pod::Pod, ui::InputContext};

use self::logs_keeper::LogsKeeper;

mod logs_keeper;
pub struct App {
    running: bool,
    namespace: String,
    pods: Option<Vec<Pod>>,
    pod_logs: LogsKeeper,
}

impl App {
    pub fn new(namespace: String) -> App {
        let running = true;
        let pods = Some(load_all_pods(&namespace).expect("couldn't load initial pods"));
        let pod_logs = LogsKeeper::default();

        App {
            running,
            namespace,
            pods,
            pod_logs,
        }
    }

    fn exit(&mut self) {
        self.running = false;
    }

    pub fn running(&self) -> &bool {
        &self.running
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn pods(&self) -> Option<&Vec<Pod>> {
        self.pods.as_ref()
    }

    pub fn get_pod(&self, index: usize) -> &Pod {
        &self
            .pods
            .as_ref()
            .expect("pods are loaded before the app right now")[index]
    }

    pub fn get_pod_name(&self, index: usize) -> &str {
        &self.get_pod(index).name
    }

    pub fn pod_logs(&self) -> Option<&Vec<String>> {
        self.pod_logs.logs()
    }

    pub fn get_logged_pod_name(&self) -> Option<String> {
        self.pod_logs.pod_name().map(|it| it.to_owned())
    }

    pub fn get_pods_number(&self) -> usize {
        self.pods.as_ref().map(|it| it.len()).unwrap_or(0)
    }

    pub fn take_action(&mut self, action: InputAction, context: InputContext) {
        match action {
            InputAction::Quit => {
                self.exit();
            }
            InputAction::FetchLogs => {
                let pod_name = self.get_pod_name(context.selected_pod_index);
                self.pod_logs =
                    LogsKeeper::new(pod_name.to_owned(), fetch_logs(pod_name, &self.namespace))
            }
            _ => {}
        }
    }
}

fn fetch_logs(pod_name: &str, namespace: &str) -> Vec<String> {
    load_logs(pod_name, namespace).expect("todo load logs")
}
