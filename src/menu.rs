#[derive(Debug, Copy, Clone)]
pub enum MenuItem {
    Home,
    Pods,
}

impl From<&MenuItem> for usize {
    fn from(input: &MenuItem) -> Self {
        match input {
            MenuItem::Home => 0,
            MenuItem::Pods => 1,
        }
    }
}