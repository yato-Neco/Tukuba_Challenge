use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use flacon::{FlaCon};
use gps::GPS;
use mytools::time_sleep;
use robot_gpio::Moter;
use super::mode::{AutoModule,KeyModule,KeyEvents,AutoEvents, RasPicoKeyModule};
use std::{error::Error, io};
use tui::widgets::Paragraph;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};

pub fn start() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    print!("\x1b[2J");

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();
    terminal
}

pub fn key_ui<B: Backend>(f: &mut Frame<B>, flacn: &FlaCon<KeyModule,KeyEvents>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(f.size());

    let left_block = Paragraph::new(format!(
        "is_move:{}\n\nis_emergency_stop: {}\n\norder: {:x}\n ",
        flacn.event.is_move,
        flacn.event.is_emergency_stop_lv0,
        flacn.event.order,
    ))
    .block(Block::default().borders(Borders::ALL))
    .alignment(tui::layout::Alignment::Left);
    f.render_widget(left_block, chunks[0]);

    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let middle_top_block = Paragraph::new("Middle01")
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(middle_top_block, middle_chunks[0]);

    let center_block = Paragraph::new("Middle02")
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(center_block, middle_chunks[1]);

    let middle_bottom = Paragraph::new("Middle03")
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(middle_bottom, middle_chunks[2]);

    
    let right_block = Paragraph::new(
        format!("{:?}\n",
        flacn.module.moter_controler,
        //flacn.module
    ))
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Left);
    f.render_widget(right_block, chunks[2]);
}


pub fn auto_ui<B: Backend>(f: &mut Frame<B>, flacn: AutoEvents,module:GPS) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(f.size());

    let left_block = Paragraph::new(format!(
        "is_move:{}\nis_emergency_stop: {}\norder: {:x} \nis_break {}\nazimuth: {}\nfirst_time: {}\nin_waypoint: {} \nnum_sat: {:?}\ncontinue: {} \nmaneuver: {}",
        flacn.is_move.get(),
        flacn.is_emergency_stop_lv0.get(),
        flacn.order.get(),
        flacn.is_break,
        module.azimuth,
        flacn.first_time,
        module.in_waypoint,
        module.num_sat,
        flacn.is_continue,
        flacn.maneuver,

    ))
    .block(Block::default().borders(Borders::ALL))
    .alignment(tui::layout::Alignment::Left);
    f.render_widget(left_block, chunks[0]);

    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let middle_top_block = Paragraph::new("Middle01")
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(middle_top_block, middle_chunks[0]);

    let center_block = Paragraph::new("Middle02")
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(center_block, middle_chunks[1]);

    let middle_bottom = Paragraph::new("Middle03")
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(middle_bottom, middle_chunks[2]);

    
    let right_block = Paragraph::new(
        format!("now: {:?} \nis_fix: {:?} \n{:?} \n{:?}\n{:?}",
        module.nowpotion,
        module.is_fix,
        module.latlot,
        module.next_latlot,
        module.rome,
    ))
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Left);
    f.render_widget(right_block, chunks[2]);
}




pub fn raspico_key_ui<B: Backend>(f: &mut Frame<B>, flacn: &FlaCon<RasPicoKeyModule,KeyEvents>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(f.size());

    let left_block = Paragraph::new(format!(
        "is_move:{}\n\nis_emergency_stop: {}\n\norder: {:x}\n ",
        flacn.event.is_move,
        flacn.event.is_emergency_stop_lv0,
        flacn.event.order,
    ))
    .block(Block::default().borders(Borders::ALL))
    .alignment(tui::layout::Alignment::Left);
    f.render_widget(left_block, chunks[0]);

    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let middle_top_block = Paragraph::new("Middle01")
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(middle_top_block, middle_chunks[0]);

    let center_block = Paragraph::new("Middle02")
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(center_block, middle_chunks[1]);

    let middle_bottom = Paragraph::new("Middle03")
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(middle_bottom, middle_chunks[2]);

    
    let right_block = Paragraph::new(
        format!("{:?}\n",
        "..."
        //flacn.module.raspico_controler,
        //flacn.module
    ))
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Left);
    f.render_widget(right_block, chunks[2]);
}


pub fn end(terminal:&mut Terminal<CrosstermBackend<std::io::Stdout>>) {
    time_sleep(0, 5000);

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();
    terminal.show_cursor().unwrap();
}

#[test]
fn test() {
    let mut tmp = start();
    tmp.clear().unwrap();
}