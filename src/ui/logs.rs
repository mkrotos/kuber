use tui::{
    layout::Corner::BottomLeft,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

pub struct LoggerWidget {
    pod_name: Option<String>,
    offset: usize,
}

impl LoggerWidget {
    pub fn new(pod_name: Option<String>) -> LoggerWidget {
        LoggerWidget {
            pod_name,
            offset: 0,
        }
    }

    pub fn should_update_widget(&self, actual_logged_pod_name: Option<String>) -> bool {
        self.pod_name != actual_logged_pod_name
    }

    pub fn page_up(&mut self, _chunk_height: u16) {
        self.offset += 1
    }

    pub fn page_down(&mut self, _chunk_height: u16) {
        //temp
        if self.offset != 0 {
            self.offset -= 1
        }
    }

    pub fn end(&mut self) {
        self.offset = 0;
    }

    pub fn render_pod_logs<'a>(
        &self,
        logs_opt: Option<&'a Vec<String>>,
        chunk_width: &u16,
    ) -> List<'a> {
        let pods = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(match &self.pod_name {
                Some(pod_name) => format!("Logs: {pod_name}"),
                None => "Logs".to_string(),
            })
            .border_type(BorderType::Plain);

        let items: Vec<_> = match logs_opt {
            Some(logs) => {
                let length = logs.len();
                let log_list: Vec<_> = logs
                    .iter()
                    .take(length - self.offset)
                    .rev()
                    .take(50 + self.offset)
                    .map(|it| default_list_item(it, chunk_width))
                    .collect();
                log_list
            }
            None => vec![default_list_item(
                "Press 'Enter' to load pod logs.",
                chunk_width,
            )],
        };

        let list = List::new(items).block(pods).start_corner(BottomLeft);
        list
    }
}

fn default_list_item<'a>(value: &'a str, chunk_width: &u16) -> ListItem<'a> {
    let spans: Vec<_> = textwrap::wrap(value, *chunk_width as usize)
        .into_iter()
        .map(|it| {
            let style = choose_style(&it);
            Span::styled(it, style)
        })
        .map(|it| Spans::from(it))
        .collect();

    ListItem::new(spans)
}

fn choose_style(log_line: &str) -> Style {
    let st = Style::default();
    match log_line {
        s if s.contains("FATAL") => st.fg(Color::Red),
        s if s.contains("ERROR") => st.fg(Color::Red),
        s if s.contains("WARN") => st.fg(Color::LightYellow),
        s if s.contains("INFO") => st.fg(Color::White),
        s if s.contains("DEBUG") => st.fg(Color::Gray),
        _ => st,
    }
}
