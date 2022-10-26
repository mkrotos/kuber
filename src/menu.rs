#[derive(Debug, Copy, Clone)]
enum MenuItem {
    Home,
    Settings,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> Self {
        match input {
            MenuItem::Home => 0,
            MenuItem::Settings => 1,
        }
    }
}