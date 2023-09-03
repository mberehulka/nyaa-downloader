use std::{path::{Path, PathBuf}, io::Write};
use crossterm::event::{Event, self, KeyCode, KeyEventKind, MouseEvent, MouseEventKind, MouseButton, KeyEvent};
use ratatui::{
    prelude::{Backend, Rect, Layout, Direction, Constraint},
    widgets::{Paragraph, Block, Borders}, Frame, style::{Style, Color}, Terminal, text::{Line, Span}
};
use reqwest::blocking::Client;

pub fn _read<B: Backend>(terminal: &mut Terminal<B>, prompt: &str) -> String {
    let mut res = String::new();
    loop {
        terminal.draw(|f| {
            if res.len() == 0 {
                let res = "Start typing...";
                f.render_widget(
                    Paragraph::new(res)
                        .style(Style::default().fg(ratatui::style::Color::DarkGray))
                        .alignment(ratatui::prelude::Alignment::Center)
                        .block(
                            Block::default().borders(Borders::ALL).title(prompt)
                                .style(Style::default().fg(ratatui::style::Color::White))
                        ),
                    crate::utils::center_chunks(f, (res.len()as u16 + 4).max(prompt.len()as u16+4), 3)
                )
            } else {
                f.render_widget(
                    Paragraph::new(res.clone())
                        .alignment(ratatui::prelude::Alignment::Center)
                        .block(Block::default().borders(Borders::ALL).title(prompt)),
                    crate::utils::center_chunks(f, (res.len()as u16 + 4).max(prompt.len()as u16+4), 3)
                )
            }
        }).unwrap();
        if let Event::Key(key) = event::read().unwrap() {
            if let KeyEventKind::Press = key.kind {
                match key.code {
                    KeyCode::Char(c) => res.push(c),
                    KeyCode::Backspace => {res.pop();},
                    KeyCode::Enter => break,
                    _ => {}
                }
            }
        }
    }
    res
}

pub fn center_chunks<B: Backend>(f: &Frame<B>, width: u16, height: u16) -> Rect {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((f.size().height/2).saturating_sub(height/2)),
            Constraint::Length(height),
            Constraint::Length((f.size().height/2).saturating_sub(height/2))
        ])
        .split(f.size());
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((f.size().width/2).saturating_sub(width/2)),
            Constraint::Length(width),
            Constraint::Length((f.size().width/2).saturating_sub(width/2))
        ])
        .split(v[1])[1]
}

pub fn select<B: Backend>(terminal: &mut Terminal<B>, title: &str, values: &[&str]) -> Option<usize> {
    if values.len() == 0 {
        let text = "Nothing found...";
        while match event::read().unwrap() {
            Event::Key(KeyEvent { kind: KeyEventKind::Press, .. }) |
            Event::Mouse(MouseEvent { kind: MouseEventKind::Up(_), .. }) => false,
            _ => true
        } {
            terminal.draw(|f| f.render_widget(
                Paragraph::new(vec![Line::from(text)]),
                crate::utils::center_chunks(f, text.len()as u16, 1)
            )).unwrap();
        }
        return None
    }
    let mut current_line = 0;
    loop {
        terminal.draw(|f| {
            let h2 = f.size().height / 2 - 1;
            current_line = (values.len()-1).min(current_line);
            let lines = values.into_iter().enumerate().map(|(line, v)|
                Line::from(vec![
                    Span::styled("> ", Style::default().fg(if line == current_line {Color::Reset} else {Color::Black})),
                    Span::raw(*v)
                ])
            ).collect::<Vec<_>>();
            let scroll = if current_line as u16 > h2 { current_line as u16 - h2  } else { 0 };
            f.render_widget(
                Paragraph::new(lines)
                    .style(Style::default().fg(Color::White))
                    .alignment(ratatui::prelude::Alignment::Left)
                    .block(
                        Block::default().borders(Borders::ALL).title(title)
                            .style(Style::default().fg(Color::DarkGray))
                    )
                    .scroll((scroll, 0)),
                crate::utils::center_chunks(f, f.size().width, f.size().height)
            )
        }).unwrap();
        match event::read().unwrap() {
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Esc, .. }) => return None,
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Up, .. }) |
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Char('w'), .. }) |
            Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollUp, .. }) =>
                current_line = current_line.saturating_sub(1),
                Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Down, .. }) |
                Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Char('s'), .. }) |
            Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollDown, .. }) =>
                current_line += 1,
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Left, .. }) |
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Right, .. }) |
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Char(_), .. }) |
            Event::Mouse(MouseEvent { kind: MouseEventKind::Up(MouseButton::Left), .. }) |
            Event::Mouse(MouseEvent { kind: MouseEventKind::Up(MouseButton::Right), .. }) =>
                return Some(current_line),
            _ => {}
        }
    }
}

pub fn download_file(link: &str) -> PathBuf {
    let exe = std::env::current_exe().unwrap();
    let mut path = Path::new(exe.as_path().parent().unwrap()).join("torrents/");
    std::fs::create_dir_all(&path).unwrap();
    path = path.join(link.split('/').last().unwrap());
    let content = Client::new().get(link).send().unwrap().bytes().unwrap();
    std::fs::OpenOptions::new()
        .create(true).read(true).write(true).truncate(true)
        .open(&path).unwrap()
        .write_all(&content).unwrap();
    path.to_path_buf()
}