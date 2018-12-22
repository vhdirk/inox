use structopt;

use log;
use env_logger;

use serde;
use serde_derive;


use std::io;
use std::fs::DirBuilder;
use std::path::PathBuf;

use dirs;

use structopt::StructOpt;
use structopt::clap::{App, Arg};

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

mod util;

use crate::util::event::{Event, Events};

struct EnamelApp {
    size: Rect,
}

impl Default for EnamelApp {
    fn default() -> EnamelApp {
        EnamelApp {
            size: Rect::default(),
        }
    }
}

/// Init logger.
fn init() {
    use std::sync::{Once, ONCE_INIT};

    static START: Once = ONCE_INIT;

    START.call_once(|| {
        env_logger::init();
    });
}

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(help="The configuration file to load.", parse(from_os_str))]
    config: Option<PathBuf>,
    #[structopt(help="Print help message.")]
    help: bool,
}

impl Default for Args{
    fn default() -> Self {
        Args{
            config: Some(default_config_path()),
            help: false
        }
    }
}

fn default_config_path() -> PathBuf{
    let mut default_config = dirs::config_dir().unwrap();
    default_config.push("enamel");
    default_config.push("config");
    default_config.set_extension("toml");
    return default_config;
}


fn main() -> Result<(), io::Error> {
    init();

    let mut default_config = dirs::config_dir().unwrap();
    default_config.push("enamel");

    DirBuilder::new()
        .recursive(true)
        .create(default_config.to_str().unwrap()).unwrap();

    // let args = Args::from_args();

    let _args = App::new("Enamel")
        .version("0.0.1")
        .author("Dirk Van Haerenborgh <vhdirk@gmail.com>")
        .about("An email client with notmuch rust.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .default_value(default_config.to_str().unwrap())
                .help(
                    "The configuration file to load.",
                ),
        )
        .get_matches();


    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Create default app state
    let mut app = EnamelApp::default();

    // Setup event handlers
    let events = Events::new();

    loop {
        let size = terminal.size()?;
        if app.size != size {
            terminal.resize(size)?;
            app.size = size;
        }

        terminal.draw(|mut f| {
            // Wrapping block for a group
            // Just draw the block and the group on the same area and build the group
            // with at least a margin of 1
            Block::default().borders(Borders::ALL).render(&mut f, size);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(4)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(app.size);
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[0]);
                Block::default()
                    .title("With background")
                    .title_style(Style::default().fg(Color::Yellow))
                    .style(Style::default().bg(Color::Green))
                    .render(&mut f, chunks[0]);
                Block::default()
                    .title("Styled title")
                    .title_style(
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Red)
                            .modifier(Modifier::Bold),
                    ).render(&mut f, chunks[1]);
            }
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[1]);
                Block::default()
                    .title("With borders")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[0]);
                Block::default()
                    .title("With styled borders")
                    .border_style(Style::default().fg(Color::Cyan))
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .render(&mut f, chunks[1]);
            }
        })?;

        match events.next() {
            Ok(Event::Input(key)) => if key == Key::Char('q') {
                break;
            },
            _ => {}
        }
    }
    Ok(())
}