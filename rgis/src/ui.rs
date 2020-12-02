use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget, List, ListItem};
use tui::Terminal;
use tui::style::{Color, Modifier, Style};

pub fn draw() -> std::io::Result<()> {
    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear()?;
    terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(100),
                        // Constraint::Percentage(80),
                        // Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            f.render_widget(layers_list(), chunks[0]);
        })
}

fn layers_list() -> List<'static> {
    let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
    let list = List::new(items)
        .block(Block::default().title("Layers").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    list
}
