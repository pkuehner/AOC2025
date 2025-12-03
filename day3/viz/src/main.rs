use core::convert::Into;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App<'a> {
    current_num: &'a str,
    values: Vec<i64>,
    start_pos: Vec<usize>,
    cursor_pos: Vec<usize>,
    exit: bool,
    counter: usize,
}

impl<'a> Widget for &App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Advent of Code 2025 - Day 3 - Part 2".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut string_to_draw: Vec<Span> = Vec::new();
        if self.start_pos[self.counter] != 99999999 {
            if self.start_pos[self.counter] > 0 as usize {
                string_to_draw.push(
                    self.current_num
                        [self.start_pos[self.counter] - 1..self.start_pos[self.counter]]
                        .blue(),
                );

                string_to_draw.push(
                    self.current_num
                        [self.start_pos[self.counter]..self.cursor_pos[self.counter] + 1]
                        .red(),
                );
            }
            string_to_draw.push(
                self.current_num[self.cursor_pos[self.counter]..self.cursor_pos[self.counter] + 1]
                    .green()
                    .on_gray(),
            );
            string_to_draw.push(self.current_num[self.cursor_pos[self.counter] + 1..].green());
        }
        let counter_text = vec![
            Line::from(string_to_draw),
            self.values[self.counter].to_string().red().into(),
        ];

        Paragraph::new(counter_text)
            .centered()
            .block(block.clone())
            .render(area, buf);
    }
}

impl<'a> App<'a> {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let mut app = Self::default();
        app.current_num = "4346343235149456543445233353534244533333333343433259333326337334334333438332533343452433223352443324";
        app.values = Vec::new();
        app.cursor_pos = Vec::new();
        app.start_pos = Vec::new();
        let mut digits: Vec<i64> = Vec::new();
        let mut value = 0;
        for c in app.current_num.chars() {
            let digit = (c.to_string()).parse::<i64>().unwrap();
            digits.push(digit);
        }
        let mut max_pos: usize = 0;

        for i in 0..12 {
            let mut max = -1;
            for j in max_pos..(digits.len() - (12 - i - 1)) {
                app.cursor_pos.push(j);
                app.values.push(value);

                if digits[j] > max {
                    max = digits[j];
                    max_pos = j + 1;
                }

                app.start_pos.push(max_pos);

                if max == 9 {
                    break;
                }
            }
            value *= 10;
            value += max;
        }
        app.values.push(value);
        app.start_pos.push(99999999);
        app.cursor_pos.push(99999999);
        app
    }

    /// Run the application's main loop.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if self.counter < self.cursor_pos.len() - 1 {
                self.counter += 1;
            }
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(150))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    /// Set running to false to quit the application.
    fn exit(&mut self) {
        self.exit = true;
    }
}
