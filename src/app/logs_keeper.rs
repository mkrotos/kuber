pub struct LogsKeeper {
    pod_name: Option<String>,
    logs: Option<Vec<String>>,
}

impl LogsKeeper {
    pub fn default() -> LogsKeeper {
        LogsKeeper {
            pod_name: None,
            logs: None,
        }
    }

    pub fn new(pod_name: String, logs: Vec<String>) -> LogsKeeper {
        LogsKeeper {
            pod_name: Some(pod_name),
            logs: Some(logs),
        }
    }

    pub fn logs(&self) -> Option<&Vec<String>> {
        self.logs.as_ref()
    }

    pub fn pod_name(&self) -> Option<&String> {
        self.pod_name.as_ref()
    }
}
