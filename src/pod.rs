#[derive(Clone)]
pub struct Pod {
    pub name: String,
    pub ready: String,
    pub status: String,
    pub restarts: String,
    pub age: String,
}

impl Pod {
    pub fn default() -> Pod {
        Pod {
            name: "some-pod".to_string(),
            ready: "1/1".to_string(),
            status: "Ready".to_string(),
            restarts: "0".to_string(),
            age: "1d".to_string(),
        }
    }
    pub fn default2() -> Pod {
        Pod {
            name: "some-pod-2".to_string(),
            ready: "1/2".to_string(),
            status: "Broken".to_string(),
            restarts: "0".to_string(),
            age: "1d".to_string(),
        }
    }
}
