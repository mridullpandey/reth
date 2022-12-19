use std::fmt::Display;

use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::*;
use tui::terminal::Frame;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::widgets::{ListItem, ListState};

#[derive(Clone, Debug)]
pub struct List<T> {
    title: String,
    pub items: Vec<T>,
    pub state: ListState,
}

impl<T: Display> List<T> {
    pub fn new(title: &str, items: Vec<T>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self { title: title.to_owned(), items, state }
    }

    /// Renders the component and highlights accordingly.
    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<'_, B>,
        area: Rect,
        focused: bool,
    ) -> eyre::Result<()> {
        // Convert the items to `ListItem`s
        let items =
            self.items.iter().map(|item| ListItem::new(format!("{}", item))).collect::<Vec<_>>();

        // Create the widget
        let list = tui::widgets::List::new(items)
            .block(Block::default().title(self.title.clone()).borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))
            .highlight_symbol(">>");

        // Render it
        f.render_stateful_widget(list, area, &mut self.state);
        Ok(())
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // Select the previous item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select(&mut self, index: usize) {
        self.state.select(Some(index));
    }

    // Unselect the currently selected item if any. The implementation of `ListState` makes
    // sure that the stored offset is also reset.
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
