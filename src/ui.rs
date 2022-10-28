use std::{
    io::{self, Stdout},
    time::Duration,
};

use crate::{
    app::App,
    input::{self, event_loop::EventLoop},
};
use crossterm::terminal::disable_raw_mode;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    terminal::CompletedFrame,
    widgets::ListState,
    Terminal,
};

mod footer;
mod header;
mod main_body;

pub struct UI<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    pod_list_state: ListState,
    event_loop: EventLoop,
    app: &'a mut App,
}

impl<'a> UI<'a> {
    pub fn init(app: &mut App) -> Result<UI, Box<dyn std::error::Error>> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        let pod_list_state = ListState::default();

        let tick_rate = Duration::from_millis(200);
        let event_loop = EventLoop::start(tick_rate);

        Ok(UI {
            terminal,
            pod_list_state,
            event_loop,
            app,
        })
    }

    pub fn start(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.clear()?;

        while *self.app.running() {
            self.draw_screen()?;
            self.handle_input()?;
        }

        self.terminal.clear()?;
        disable_raw_mode()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    fn draw_screen(&mut self) -> Result<CompletedFrame, io::Error> {
        self.terminal.draw(|rect| {
            let pods = self.app.pods().expect("todo");
            let namespace = self.app.namespace();

            let size = rect.size();
            let chunks = split_screen_vertically(size);

            // Draw header and footer
            rect.render_widget(header::render_info(namespace), chunks[0]);
            rect.render_widget(footer::render_footer(), chunks[2]);

            let pods_chunks = split_body_horizontally(chunks[1]);
            let (details_chunk, logs_chunk) = split_pod_details_vertically(pods_chunks[1]);

            let pods_list = main_body::render_pods_list(pods);
            let selected_pod = self.app.get_selected_pod();
            let pod_details = main_body::render_pod_details(selected_pod.clone());
            self.pod_list_state.select(self.app.selected_pod_index);

            // let logs_chunk_size = (logs_chunk.x, logs_chunk.y)
            let pod_logs = main_body::render_pod_logs(self.app.get_logged_pod_name(), self.app.pod_logs());

            // Draw main body
            rect.render_stateful_widget(pods_list, pods_chunks[0], &mut self.pod_list_state);
            rect.render_widget(pod_details, details_chunk);
            rect.render_widget(pod_logs, logs_chunk)
        })
    }

    fn handle_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(action) = input::map_input(self.event_loop.next()?) {
            self.app.take_action(action);
        };

        Ok(())
    }
}

fn split_screen_vertically(size: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);
    chunks
}

fn split_body_horizontally(chunk: Rect) -> Vec<Rect> {
    let pods_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunk);
    pods_chunks
}

fn split_pod_details_vertically(chunk: Rect) -> (Rect, Rect) {
    let details_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunk);
    (details_chunk[0], details_chunk[1])
}
