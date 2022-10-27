use tui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render_info(namespace: &str) -> Paragraph<'static> {
    let info = Paragraph::new(format!("Namespace: {namespace}"))
        .style(Style::default().fg(Color::White))
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
