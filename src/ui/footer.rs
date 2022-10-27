use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render_footer() -> Paragraph<'static> {
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
