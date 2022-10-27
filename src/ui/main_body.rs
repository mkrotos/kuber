use crate::pod::Pod;

use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, Row, Table},
};

pub fn render_pods_list<'a>(pod_list: &'a Vec<Pod>) -> List<'a> {
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

    let list = List::new(items).block(pods).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    list
}

pub fn render_pod_details<'a>(selected_pod: Pod) -> Table<'a> {
    let rows = vec![Row::new(vec![
        Cell::from(Span::raw(selected_pod.name)),
        Cell::from(Span::raw(selected_pod.ready)),
        Cell::from(Span::raw(selected_pod.status)),
        Cell::from(Span::raw(selected_pod.restarts)),
        Cell::from(Span::raw(selected_pod.age)),
    ])];

    let header_row = Row::new(vec![
        header_cell("Name"),
        header_cell("Ready"),
        header_cell("Status"),
        header_cell("Restarts"),
        header_cell("Age"),
    ]);

    let pod_detail = Table::new(rows)
        .header(header_row)
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

    pod_detail
}

fn header_cell(title: &str) -> Cell {
    Cell::from(Span::styled(
        title,
        Style::default().add_modifier(Modifier::BOLD),
    ))
}
