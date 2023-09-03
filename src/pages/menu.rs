use crossterm::event::{Event, self, KeyEventKind, KeyCode, MouseEvent, MouseEventKind, MouseButton, KeyEvent};
use ratatui::{prelude::Backend, Terminal, widgets::{Paragraph, Borders, Block}, text::{Line, Span}, style::{Style, Color}};

use crate::anilist::User;

pub fn show<B: Backend>(user: &User, terminal: &mut Terminal<B>) {
    let mut watching = user.watching();
    
    let mut current_line = 0;
    loop {
        terminal.draw(|f| {
            let h2 = f.size().height / 2 - 1;
            current_line = (watching.len()-1).min(current_line);
            let lines = watching.iter().enumerate().map(|(line, anime)|
                Line::from(vec![
                    Span::styled("> ", Style::default().fg(if line == current_line {Color::Reset} else {Color::Black})),
                    Span::raw(format!(
                        "{} {}/{}",
                        anime.name,
                        anime.progress,
                        anime.episodes
                    ))
                ])
            ).collect::<Vec<_>>();
            let scroll = if current_line as u16 > h2 { current_line as u16 - h2  } else { 0 };
            f.render_widget(
                Paragraph::new(lines)
                    .style(Style::default().fg(Color::White))
                    .alignment(ratatui::prelude::Alignment::Left)
                    .block(
                        Block::default().borders(Borders::ALL).title("Watching")
                            .style(Style::default().fg(Color::DarkGray))
                    )
                    .scroll((scroll, 0)),
                crate::utils::center_chunks(f, f.size().width, f.size().height)
            )
        }).unwrap();
        match event::read().unwrap() {
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Esc, .. }) => break,
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Up, .. }) |
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Char('w'), .. }) |
            Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollUp, .. }) =>
                current_line = current_line.saturating_sub(1),
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Down, .. }) |
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Char('s'), .. }) |
            Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollDown, .. }) =>
                current_line += 1,
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Right, .. }) |
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Char('d'), .. }) |
            Event::Mouse(MouseEvent { kind: MouseEventKind::Up(MouseButton::Right), .. }) => {
                user.update_anime_progress(&watching[current_line], 1);
                watching = user.watching()
            },
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Left, .. }) |
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Char('a'), .. }) |
            Event::Mouse(MouseEvent { kind: MouseEventKind::Up(MouseButton::Left), .. }) => {
                user.update_anime_progress(&watching[current_line], -1);
                watching = user.watching()
            },
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Char(' '), .. }) |
            Event::Key(KeyEvent { kind: KeyEventKind::Press, code: KeyCode::Enter, .. }) => {
                if crate::nyaa::watch_next_episode(terminal, &watching[current_line]) {
                    user.update_anime_progress(&watching[current_line], 1);
                    watching = user.watching()
                }
            },
            _ => {}
        }
    }
}