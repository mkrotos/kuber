use crate::{input::InputAction, load_all_pods, load_logs, pod::Pod};

use self::logs_keeper::LogsKeeper;

mod logs_keeper;
pub struct App {
    running: bool,
    pub selected_pod_index: Option<usize>,
    namespace: String,
    pods: Option<Vec<Pod>>,
    pod_logs: LogsKeeper,
}

impl App {
    pub fn new(namespace: String) -> App {
        let running = true;
        let selected_pod_index = Some(0);
        let pods = Some(load_all_pods(&namespace).expect("couldn't load initial pods"));
        let pod_logs = LogsKeeper::default();

        App {
            running,
            selected_pod_index,
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

    pub fn get_selected_pod(&self) -> &Pod {
        &self.pods.as_ref().unwrap()[self.selected_pod_index.expect("todo")]
    }

    pub fn get_selected_pod_name(&self) -> &str {
        &self.get_selected_pod().name
    }

    pub fn pod_logs(&self) -> Option<&Vec<String>> {
        self.pod_logs.logs()
    }

    pub fn get_logged_pod_name(&self) -> Option<String> {
        self.pod_logs.pod_name().map(|it| it.to_owned())
    }

    pub fn take_action(&mut self, action: InputAction) {
        match action {
            InputAction::Quit => {
                self.exit();
            }
            InputAction::NextPod => {
                self.select_next_pod();
            }
            InputAction::PreviousPod => {
                self.select_previous_pod();
            }
            InputAction::FetchLogs => {
                let pod_name = self.get_selected_pod_name();
                self.pod_logs =
                    LogsKeeper::new(pod_name.to_owned(), fetch_logs(pod_name, &self.namespace))
            }
        }
    }

    fn select_next_pod(&mut self) {
        let pods_number = self.pods().map(|it| it.len()).unwrap_or(0);

        if let Some(selected) = self.selected_pod_index {
            if selected >= pods_number - 1 {
                self.selected_pod_index = Some(0);
            } else {
                self.selected_pod_index = Some(selected + 1);
            }
        }
    }

    fn select_previous_pod(&mut self) {
        let pods_number = self.pods().map(|it| it.len()).unwrap_or(0);

        if let Some(selected) = self.selected_pod_index {
            if selected > 0 {
                self.selected_pod_index = Some(selected - 1);
            } else {
                self.selected_pod_index = Some(pods_number - 1);
            }
        }
    }
}

fn fetch_logs(pod_name: &str, namespace: &str) -> Vec<String> {
    load_logs(pod_name, namespace).expect("todo load logs")
}
