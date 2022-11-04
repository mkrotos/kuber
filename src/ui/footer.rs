use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render_about() -> Paragraph<'static> {
    let about = Paragraph::new("Kuber v0.1.0")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("About")
                .border_type(BorderType::Plain),
        );
    about
}

pub fn render_keys() -> Paragraph<'static> {
    let about = Paragraph::new(
        "Up/Down - select pod | Enter - fetch logs | PageUp/PageDown/End - scroll logs",
    )
    .style(Style::default().fg(Color::White))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Control")
            .border_type(BorderType::Plain),
    );
    about
}
