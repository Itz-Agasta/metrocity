use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::config::Config;
use crate::scene::Scene;

pub struct Engine {
    fps: u32,
    #[allow(dead_code)]
    show_cursor: bool,
}

impl Engine {
    pub fn new(config: &Config) -> Self {
        Self {
            fps: config.engine.fps.max(1),
            show_cursor: false,
        }
    }

    /// Run the engine with the given scene. Blocks until exit.
    pub fn run(&mut self, scene: &mut dyn Scene) -> Result<(), Box<dyn std::error::Error>> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Initialize scene with terminal size
        let size = terminal.size()?;
        let theme = crate::theme::Theme::default(); // TODO: accept theme from config
        scene.init(size.width, size.height, &theme);

        // Main loop
        let tick_rate = Duration::from_millis((1000 / self.fps) as u64);
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        break; // any key exits
                    }
                    Event::Resize(_, _) => {
                        let size = terminal.size()?;
                        scene.init(size.width, size.height, &theme);
                    }
                    _ => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                let dt = last_tick.elapsed().as_secs_f64();
                last_tick = Instant::now();
                scene.update(dt.min(0.1)); // cap dt to prevent jumps
                terminal.draw(|f| {
                    let area = f.area();
                    let buf = f.buffer_mut();
                    scene.draw(area, buf);
                })?;
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    }
}
