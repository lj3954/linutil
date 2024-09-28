use std::borrow::Cow;

use crate::{
    float::{FloatContent, FloatEvent},
    hint::Shortcut,
};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Alignment,
    prelude::*,
    widgets::{Block, Borders, Clear, List},
};

pub struct ConfirmPrompt {
    pub names: Box<[String]>,
    scroll: usize,
}

impl ConfirmPrompt {
    pub fn new(names: &[&str]) -> Self {
        let names = names
            .iter()
            .zip(1..)
            .map(|(name, n)| format!("{n}. {name}"))
            .collect();

        Self { names, scroll: 0 }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll < self.names.len() - 1 {
            self.scroll += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }
}

impl FloatContent for ConfirmPrompt {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Confirm selections ")
            .title_alignment(Alignment::Center)
            .title_style(Style::default().bold())
            .style(Style::default());

        frame.render_widget(block.clone(), area);

        let inner_area = block.inner(area);

        let paths_text = self
            .names
            .iter()
            .skip(self.scroll)
            .map(|p| {
                let span = Span::from(Cow::<'_, str>::Borrowed(p));
                Line::from(span).style(Style::default())
            })
            .collect::<Text>();

        frame.render_widget(Clear, inner_area);
        frame.render_widget(List::new(paths_text), inner_area);
    }

    fn handle_key_event(&mut self, key: &KeyEvent) -> FloatEvent {
        use KeyCode::*;
        match key.code {
            Char('y') | Char('Y') => return FloatEvent::ConfirmSelection,
            Char('n') | Char('N') | Esc => return FloatEvent::AbortConfirmation,
            Char('j') => self.scroll_down(),
            Char('k') => self.scroll_up(),
            _ => {}
        };
        FloatEvent::None
    }

    fn is_finished(&self) -> bool {
        true
    }

    fn get_shortcut_list(&self) -> (&str, Box<[Shortcut]>) {
        (
            "Confirmation prompt",
            Box::new([
                Shortcut::new("Continue", ["Y", "y"]),
                Shortcut::new("Abort", ["N", "n"]),
                Shortcut::new("Scroll up", ["j"]),
                Shortcut::new("Scroll down", ["k"]),
                Shortcut::new("Close linutil", ["CTRL-c", "q"]),
            ]),
        )
    }
}
