use std::{io, sync::mpsc::Receiver};

use crate::{event::Event, pod::Pod};
use crossterm::{
    event::{KeyCode, KeyEvent},
    terminal::disable_raw_mode,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table},
    Terminal,
};

mod input_loop;

pub fn start(namespace: &str, pods: Vec<Pod>) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut pod_list_state = ListState::default();
    pod_list_state.select(Some(0));

    let input_receiver = input_loop::start_input_loop()?;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
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

            rect.render_widget(render_info(namespace), chunks[0]);
            rect.render_widget(render_footer(), chunks[2]);

            let pods_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(chunks[1]);
            let (left, right) = render_pods(&pod_list_state, &pods);
            rect.render_stateful_widget(left, pods_chunks[0], &mut pod_list_state);
            rect.render_widget(right, pods_chunks[1]);
        })?;

        if let Some(action) = handle_input(&input_receiver)? {
            match action {
                InputAction::Quit => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                InputAction::NextPod => {
                    if let Some(selected) = pod_list_state.selected() {
                        let amount_pods = pods.len();
                        if selected >= amount_pods - 1 {
                            pod_list_state.select(Some(0));
                        } else {
                            pod_list_state.select(Some(selected + 1));
                        }
                    }
                }
                InputAction::PreviousPod => {
                    if let Some(selected) = pod_list_state.selected() {
                        let amount_pods = pods.len();
                        if selected > 0 {
                            pod_list_state.select(Some(selected - 1));
                        } else {
                            pod_list_state.select(Some(amount_pods - 1));
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn render_info(namespace: &str) -> Paragraph<'static> {
    let info = Paragraph::new(format!("Namespace: {namespace}"))
        .style(Style::default().fg(Color::LightCyan))
        // .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Base info")
                .border_type(BorderType::Plain),
        );
    info
}

fn handle_input(
    input_receiver: &Receiver<Event<KeyEvent>>,
) -> Result<Option<InputAction>, Box<dyn std::error::Error>> {
    match input_receiver.recv()? {
        Event::Input(event) => match event.code {
            KeyCode::Char('q') => Ok(Some(InputAction::Quit)),
            KeyCode::Down => Ok(Some(InputAction::NextPod)),
            KeyCode::Up => Ok(Some(InputAction::PreviousPod)),
            _ => Ok(None),
        },
        Event::Tick => Ok(None),
    }
}

enum InputAction {
    Quit,
    NextPod,
    PreviousPod,
}

fn render_footer() -> Paragraph<'static> {
    let copyright = Paragraph::new("Kuber v0.1.0")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("About")
                .border_type(BorderType::Plain),
        );
    copyright
}

fn render_pods<'a>(pod_list_state: &ListState, pod_list: &'a Vec<Pod>) -> (List<'a>, Table<'a>) {
    let pods = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Pods")
        .border_type(BorderType::Plain);

    let items: Vec<_> = pod_list
        .iter()
        .map(|pod| {
            ListItem::new(Spans::from(vec![Span::styled(
                pod.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_pod = pod_list
        .get(
            pod_list_state
                .selected()
                .expect("there is always a selected pod"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(pods).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let pod_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_pod.name)),
        Cell::from(Span::raw(selected_pod.ready)),
        Cell::from(Span::raw(selected_pod.status)),
        Cell::from(Span::raw(selected_pod.restarts)),
        Cell::from(Span::raw(selected_pod.age)),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Ready",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Status",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Restarts",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(30),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(10),
    ]);

    (list, pod_detail)
}
