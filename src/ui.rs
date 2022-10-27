use std::{io, sync::mpsc::Receiver};

use crate::{errors::Error, event::Event, menu::MenuItem, pod::Pod};
use crossterm::{
    event::{KeyCode, KeyEvent},
    terminal::disable_raw_mode,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

mod input_loop;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut active_menu_item = MenuItem::Home;

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

            rect.render_widget(render_footer(), chunks[2]);

            let menu = render_menu(&active_menu_item);
            rect.render_widget(menu, chunks[0]);

            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::Pods => {
                    let pods_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = render_pods(&pod_list_state);
                    rect.render_stateful_widget(left, pods_chunks[0], &mut pod_list_state);
                    rect.render_widget(right, pods_chunks[1]);
                }
            }
        })?;

        if let Some(action) = handle_input(&input_receiver)? {
            match action {
                InputAction::Quit => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                InputAction::GoHome => active_menu_item = MenuItem::Home,
                InputAction::GoPods => active_menu_item = MenuItem::Pods,
                InputAction::NextPod => {
                    if let Some(selected) = pod_list_state.selected() {
                        let amount_pods = read_db().expect("can fetch pod list").len();
                        if selected >= amount_pods - 1 {
                            pod_list_state.select(Some(0));
                        } else {
                            pod_list_state.select(Some(selected + 1));
                        }
                    }
                },
                InputAction::PreviousPod => {
                    if let Some(selected) = pod_list_state.selected() {
                        let amount_pods = read_db().expect("can fetch pod list").len();
                        if selected > 0 {
                            pod_list_state.select(Some(selected - 1));
                        } else {
                            pod_list_state.select(Some(amount_pods - 1));
                        }
                    }
                },
            }
        }
    }

    Ok(())
}

fn handle_input(
    input_receiver: &Receiver<Event<KeyEvent>>,
) -> Result<Option<InputAction>, Box<dyn std::error::Error>> {
    match input_receiver.recv()? {
        Event::Input(event) => match event.code {
            KeyCode::Char('q') => Ok(Some(InputAction::Quit)),
            KeyCode::Char('h') => Ok(Some(InputAction::GoHome)),
            KeyCode::Char('p') => Ok(Some(InputAction::GoPods)),
            KeyCode::Down => Ok(Some(InputAction::NextPod)),
            KeyCode::Up => Ok(Some(InputAction::PreviousPod)),
            _ => Ok(None),
        },
        Event::Tick => Ok(None),
    }
}

enum InputAction {
    Quit,
    GoHome,
    GoPods,
    NextPod,
    PreviousPod,
}

fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Kuber",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to access pods.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

fn render_menu(active_menu_item: &MenuItem) -> Tabs<'static> {
    let menu_titles = vec!["Home", "Pods", "Something"];

    let menu = menu_titles
        .iter()
        .map(|t| {
            // Add underline to the first letter
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let tabs = Tabs::new(menu)
        .select(active_menu_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));

    tabs
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

fn read_db() -> Result<Vec<Pod>, Error> {
    let parsed: Vec<Pod> = vec![Pod::default(), Pod::default2()];
    Ok(parsed)
}

fn render_pods<'a>(pod_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let pods = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Pods")
        .border_type(BorderType::Plain);

    let pod_list = read_db().expect("can fetch pod list");
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
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ]);

    (list, pod_detail)
}
