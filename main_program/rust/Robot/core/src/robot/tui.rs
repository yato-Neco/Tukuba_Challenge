use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use flacon::FlaCon;
use gps::gps::Nav;
use gps::GPS;
use mytools::time_sleep;
use robot_gpio::Moter;
use std::fmt::format;
use std::{error::Error, io};
use tui::widgets::Paragraph;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};
use wt901::{self, WT901};

use super::mode::{key::KeyEvents, key::KeyModule, nauto::AutoEvents, nauto::AutoModule};

pub fn start() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    //print!("\x1b[2J");

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();
    terminal
}

pub fn na_ui<B: Backend>(f: &mut Frame<B>, event: &AutoEvents, nav: &Nav, wt901: &WT901) {
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
        "maneuver: {}\nis_first_time: {}\nis_flash: {}\nstart_azimuth: {}\nnext_azimuth: {}\nis_trune {}\nazimuth: {}\ngpss: {} {} {:?}",
        event.maneuver, event.is_first_time, event.is_flash, nav.start_azimuth, nav.next_azimuth, event.is_trune,event.azimuth,nav.destination_index,nav.waypoints.len(),nav.row_waypoints.get(nav.destination_index)
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
    let middle_top_block = Paragraph::new(format!(
        "gps_module: {}\nwt901_module: {}\nlidar_module: {}",
        event.is_gps_module, event.is_wt901_module, event.is_lidar_module,
    ))
    .block(Block::default().borders(Borders::ALL))
    .alignment(tui::layout::Alignment::Center);
    f.render_widget(middle_top_block, middle_chunks[0]);

    let center_block = Paragraph::new("")
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(center_block, middle_chunks[1]);

    let middle_bottom = Paragraph::new("")
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(middle_bottom, middle_chunks[2]);

    let right_block = Paragraph::new(format!(
        "GPS:\n is_fix: {:?}\n num_sat: {:?}\n lat lot: {:?}\n row data: {:?}\nWT901:\n azimath: {:?}\n row_data: {:?}",
        nav.gps_senser.is_fix, nav.gps_senser.num_sat,nav.lat_lon ,nav.gps_senser.row_data, wt901.aziment,wt901.tmp2

    ))
    .block(Block::default().borders(Borders::ALL))
    .alignment(tui::layout::Alignment::Left);
    f.render_widget(right_block, chunks[2]);
}

pub fn key_ui<B: Backend>(f: &mut Frame<B>, flacn: &FlaCon<KeyModule, KeyEvents>) {
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

    let left_block = Paragraph::new(format!(""))
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

    let right_block = Paragraph::new(format!(""))
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Left);
    f.render_widget(right_block, chunks[2]);
}

pub fn end(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) {
    time_sleep(0, 5000);

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}

#[test]
fn test() {
    let mut tmp = start();
    tmp.clear().unwrap();
}
