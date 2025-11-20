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
use crate::db::{LocationInfo, PersonInfo};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    screen: Screen,
    input: String,
    new_member: NewMemberInp,
    members: Vec<NewMemberInp>,
    exit: bool,
}

//temp struct to gather user input
#[derive(Debug, Clone, Default)]
struct NewMemberInp {
    name: String,
    /*
    id: String,
    address: String,
    city: String,
    state: String,
    zipcode: String,
    */
}

//enum for screen state
#[derive(Debug, Clone, Copy, Default)]
enum Screen {
    #[default]
    Menu,
    AddMember,
}

enum AddMemberField {
    Name,
    Id,
    Address,
    City,
    State,
    Zip,
    Done,
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

    //renders the block
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    //handles user key events
    //1 to add members, 2 to remove, 3 to request reports, 'q' to quit
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.screen {
            Screen::Menu => match key_event.code {
                KeyCode::Char('1') => {
                    self.new_member = NewMemberInp::default();
                    self.input.clear();
                    self.screen = Screen::AddMember;
                }
                KeyCode::Char('q') => self.exit = true,
                _ => {}
            },

            Screen::AddMember => match key_event.code {
                KeyCode::Esc => {
                    self.input.clear();
                    self.screen = Screen::Menu;
                }
                KeyCode::Enter => {
                    self.new_member.name = self.input.clone();
                    self.members.push(self.new_member.clone());
                    self.input.clear();
                    self.screen = Screen::Menu;
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Char(c) => {
                    self.input.push(c);
                }
                _ => {}
            },
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}

//Widget for the options presented in the UI
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.screen {
            Screen::Menu => {
                let title =
                    Line::from("----MANAGER TERMINAL----".bold().green());
                let instructions = Line::from(vec![
                    "Add Member".green().into(),
                    "<1> ".blue().bold(),
                    "Remove member".green().into(),
                    "<2> ".blue().bold(),
                    "Request manager Reports".green().into(),
                    "<3> ".blue().bold(),
                    "Quit".green().into(),
                    "<Q>".blue().bold(),
                ]);
                let block = Block::bordered()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .border_set(border::THICK)
                    .border_style(Style::default().fg(Color::Green));
                block.render(area, buf);
            }
            Screen::AddMember => {
                let block = Block::bordered()
                    .title(
                        Line::from("---ADD NEW MEMBER---".bold().green())
                            .centered(),
                    )
                    .border_set(border::THICK)
                    .border_style(Style::default().fg(Color::Green));
                let prompt =
                    Line::from("Enter new member name and press <ENTER>")
                        .fg(Color::Green);
                let input_line = Line::from(vec![
                    Span::raw(self.input.clone()),
                    Span::styled("|", Style::default().fg(Color::Green)),
                ]);

                let para =
                    Paragraph::new(vec![prompt, input_line]).block(block);
                para.render(area, buf);
            }
        }
    }
}

//driver function that initializes the terminal
pub fn run_man_term() -> io::Result<()> {
    color_eyre::install().ok();
    let mut terminal = ratatui::init();
    let app_result = App::default().run_term(&mut terminal);
    ratatui::restore();
    app_result
}
