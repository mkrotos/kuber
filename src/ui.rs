use std::{
    io::{self, Stdout},
    sync::mpsc::Receiver,
};

use crate::{event::Event, pod::Pod};
use crossterm::{event::KeyEvent, terminal::disable_raw_mode};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::ListState,
    Terminal,
};

use self::key_mapper::InputAction;

mod footer;
mod header;
mod input_loop;
mod key_mapper;
mod main_body;

pub struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    pod_list_state: ListState,
    input_receiver: Receiver<Event<KeyEvent>>,
    stop: bool,
}

impl UI {
    pub fn init() -> Result<UI, Box<dyn std::error::Error>> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        let mut pod_list_state = ListState::default();
        pod_list_state.select(Some(0));

        let input_receiver = input_loop::start_input_loop()?;

        Ok(UI {
            terminal,
            pod_list_state,
            input_receiver,
            stop: false,
        })
    }

    pub fn start(
        self: &mut Self,
        namespace: &str,
        pods: Vec<Pod>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.clear()?;

        while !self.stop {
            self.draw_screen(namespace, &pods)?;
            self.handle_input(pods.len())?;
        }

        disable_raw_mode()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    fn draw_screen(
        self: &mut Self,
        namespace: &str,
        pods: &Vec<Pod>,
    ) -> Result<tui::terminal::CompletedFrame, io::Error> {
        self.terminal.draw(|rect| {
            let size = rect.size();
            let chunks = split_screen_vertically(size);

            rect.render_widget(header::render_info(namespace), chunks[0]);
            rect.render_widget(footer::render_footer(), chunks[2]);

            let pods_chunks = split_body_horizontally(chunks[1]);

            let pods_list = main_body::render_pods_list(pods);
            let selected_pod = find_selected_pod(pods, &self.pod_list_state);

            let pod_details = main_body::render_pod_details(selected_pod.clone());
            rect.render_stateful_widget(pods_list, pods_chunks[0], &mut self.pod_list_state);
            rect.render_widget(pod_details, pods_chunks[1]);
        })
    }

    fn handle_input(self: &mut Self, pods_number: usize) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(action) = key_mapper::map_input(self.input_receiver.recv()?) {
            match action {
                InputAction::Quit => {
                    self.stop = true;
                }
                InputAction::NextPod => {
                    if let Some(selected) = self.pod_list_state.selected() {
                        if selected >= pods_number - 1 {
                            self.pod_list_state.select(Some(0));
                        } else {
                            self.pod_list_state.select(Some(selected + 1));
                        }
                    }
                }
                InputAction::PreviousPod => {
                    if let Some(selected) = self.pod_list_state.selected() {
                        if selected > 0 {
                            self.pod_list_state.select(Some(selected - 1));
                        } else {
                            self.pod_list_state.select(Some(pods_number - 1));
                        }
                    }
                }
            }
        };

        Ok(())
    }
}

fn split_body_horizontally(chunk: Rect) -> Vec<Rect> {
    let pods_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunk);
    pods_chunks
}

fn split_screen_vertically(size: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
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

fn find_selected_pod<'a>(pods: &'a Vec<Pod>, pod_list_state: &ListState) -> &'a Pod {
    let selected_pod = pods
        .get(
            pod_list_state
                .selected()
                .expect("there is always a selected pod"),
        )
        .expect("exists");
    selected_pod
}
