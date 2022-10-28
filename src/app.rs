use crate::{load_all_pods, pod::Pod, input::InputAction};

pub struct App {
    running: bool,
    pub selected_pod_index: Option<usize>,
    namespace: String,
    pods: Option<Vec<Pod>>,
}

impl App {
    pub fn new(namespace: String) -> App {
        let running = true;
        let selected_pod_index = Some(0);
        let pods = Some(load_all_pods(&namespace).expect("couldn't load initial pods"));

        App {
            running,
            selected_pod_index,
            namespace,
            pods,
        }
    }

    pub fn exit(&mut self) {
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

    pub fn take_action(&mut self, action: InputAction) {
        let pods_number = self.pods().map(|it| it.len()).unwrap_or(0);
        match action {
            InputAction::Quit => {
                self.exit();
            }
            InputAction::NextPod => {
                if let Some(selected) = self.selected_pod_index {
                    if selected >= pods_number - 1 {
                        self.selected_pod_index = Some(0);
                    } else {
                        self.selected_pod_index = Some(selected + 1);
                    }
                }
            }
            InputAction::PreviousPod => {
                if let Some(selected) = self.selected_pod_index {
                    if selected > 0 {
                        self.selected_pod_index = Some(selected - 1);
                    } else {
                        self.selected_pod_index = Some(pods_number - 1);
                    }
                }
            }
        }
    }
}
