use std::{
    error::Error,
    io::{self, Stdout},
    time::Duration,
};

use crate::{
    app::App,
    input::{self, event_loop::EventLoop, InputAction},
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
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
    pod_list_state: ListState,
    event_loop: EventLoop,
    app: &'a mut App,
    selected_pod_index: usize,
}

impl<'a> UI<'a> {
    pub fn new(app: &mut App) -> UI {
        let pod_list_state = ListState::default();
        let selected_pod_index = 0;

        let tick_rate = Duration::from_millis(200);
        let event_loop = EventLoop::start(tick_rate);

        UI {
            pod_list_state,
            event_loop,
            app,
            selected_pod_index,
        }
    }

    pub fn start(self: &mut Self) -> Result<(), Box<dyn Error>> {
        let mut terminal = prepare_terminal()?;

        while *self.app.running() {
            self.draw_screen(&mut terminal)?;
            self.handle_input()?;
        }

        restore_terminal(terminal)?;

        Ok(())
    }

    fn draw_screen<'b>(
        &'b mut self,
        terminal: &'b mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<CompletedFrame, io::Error> {
        terminal.draw(|rect| {
            let pods = self
                .app
                .pods()
                .expect("pods are loaded before the app right now");
            let namespace = self.app.namespace();

            let size = rect.size();
            let chunks = split_screen_vertically(size);

            // Draw header and footer
            rect.render_widget(header::render_info(namespace), chunks[0]);
            rect.render_widget(footer::render_footer(), chunks[2]);

            let pods_chunks = split_body_horizontally(chunks[1]);
            let (details_chunk, logs_chunk) = split_pod_details_vertically(pods_chunks[1]);

            let pods_list = main_body::render_pods_list(pods);
            let selected_pod = self.app.get_pod(self.selected_pod_index);
            let pod_details = main_body::render_pod_details(selected_pod.clone());
            self.pod_list_state.select(Some(self.selected_pod_index));

            let pod_logs = main_body::render_pod_logs(
                self.app.get_logged_pod_name(),
                self.app.pod_logs(),
                &logs_chunk.width,
            );

            // Draw main body
            rect.render_stateful_widget(pods_list, pods_chunks[0], &mut self.pod_list_state);
            rect.render_widget(pod_details, details_chunk);
            rect.render_widget(pod_logs, logs_chunk)
        })
    }

    fn handle_input(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(action) = input::map_input(self.event_loop.next()?) {
            match action {
                InputAction::NextPod => self.select_next_pod(),
                InputAction::PreviousPod => self.select_previous_pod(),
                InputAction::LogsUp => todo!(),
                InputAction::LogsDown => todo!(),
                _ => {
                    let context = InputContext {
                        selected_pod_index: self.selected_pod_index,
                    };
                    self.app.take_action(action, context)
                }
            }
        };

        Ok(())
    }

    fn select_next_pod(&mut self) {
        let pods_number = self.app.get_pods_number();

        if self.selected_pod_index >= pods_number - 1 {
            self.selected_pod_index = 0;
        } else {
            self.selected_pod_index = self.selected_pod_index + 1;
        }
    }

    fn select_previous_pod(&mut self) {
        let pods_number = self.app.get_pods_number();

        if self.selected_pod_index > 0 {
            self.selected_pod_index = self.selected_pod_index - 1;
        } else {
            self.selected_pod_index = pods_number - 1;
        }
    }
}

fn prepare_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(
    mut terminal: Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
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

pub struct InputContext {
    pub selected_pod_index: usize,
}
