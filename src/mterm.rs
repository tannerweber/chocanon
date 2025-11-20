/* File: mterm.rs
 *
 * Authors:
 * - Tanner Weber, tannerw@pdx.edu
 * - Cristian Hernandez, cristhe@pdx.edu
 * - Jethro Fernandez, jethrof@pdx.edu
 * - Torin Costales, turoczy@pdx.edu
 * - Miles Turoczy, tcostal2@pdx.edu
 *
 * Portland State University
 * Dates: October 29 to December 5
 * Course: CS 314, Fall 2025
 * Instructor: Christopher Gilmore
 */

//! Module for the manager terminal.
use crate::db::DB;
use color_eyre::install;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, List, ListItem, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    choice: u8,
    input: String,
    character_index: usize,
    exit: bool,
}

impl App {
    pub fn run_term(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Manager Terminal".bold());
        let instructions = Line::from(vec![
            "Add Member".into(),
            "Remove member".into(),
            "Request manager Reports".into(),
            "<Q>".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
    }
}
fn run_man_term() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run_term(&mut terminal);
    ratatui::restore();
    app_result
}
