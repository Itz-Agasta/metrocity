mod config;
mod engine;
mod scene;
mod scenes;
mod shell;
mod theme;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "metrocity",
    about = "A terminal screensaver — cyberpunk skyline that activates on idle",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Lock to a specific scene
    #[arg(short, long)]
    scene: Option<String>,

    /// Override the color theme
    #[arg(short, long)]
    theme: Option<String>,

    /// Force weather mode (rain, snow, clear)
    #[arg(short, long)]
    weather: Option<String>,

    /// Target frame rate
    #[arg(long, default_value_t = 30)]
    fps: u32,

    /// Seconds per scene (0 = no cycling, v1 has one scene)
    #[arg(short, long, default_value_t = 0)]
    duration: u32,
}

#[derive(Subcommand)]
enum Commands {
    /// Print shell integration snippet
    ShellInit {
        /// Shell to generate for (zsh, bash)
        shell: String,
    },
    /// List available scenes or themes
    List {
        /// What to list (scenes, themes)
        target: String,
    },
    /// Show or write configuration
    Config {
        /// Write default config file
        #[arg(long)]
        init: bool,
    },
    /// Print version info
    Version,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::ShellInit { shell }) => match shell::get_snippet(&shell) {
            Ok(snippet) => print!("{snippet}"),
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        },
        Some(Commands::List { target }) => match target.as_str() {
            "scenes" => {
                for name in scenes::names() {
                    println!("  {name}");
                }
            }
            "themes" => {
                println!("  cyberpunk   — neon-drenched cityscape");
                println!("  matrix      — green-on-black digital rain");
                println!("  synthwave   — retro sunset palette");
                println!("  dracula     — purple-pink gothic");
                println!("  sin_city    — stark black, white & red");
                println!("  default     — balanced dark tones");
            }
            _ => {
                eprintln!("Unknown list target: {target}. Use 'scenes' or 'themes'.");
                std::process::exit(1);
            }
        },
        Some(Commands::Config { init }) => {
            if init {
                config::Config::write_default()?;
                println!("Default config written to ~/.config/metrocity/config.toml");
            } else {
                let cfg = config::Config::load();
                println!("{:#?}", cfg);
            }
        }
        Some(Commands::Version) => {
            println!(
                "metrocity {}",
                option_env!("CARGO_PKG_VERSION").unwrap_or("dev")
            );
        }
        None => {
            // No subcommand — start the screensaver
            run_screensaver(&cli)?;
        }
    }

    Ok(())
}

fn run_screensaver(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::Config::load();

    // CLI overrides
    if let Some(ref theme_name) = cli.theme {
        config.appearance.theme = theme_name.clone();
    }
    if let Some(ref weather) = cli.weather {
        config.appearance.weather = weather.clone();
    }
    config.engine.fps = cli.fps;

    // TODO: pass resolved theme to engine

    // Resolve weather
    let weather = match config.appearance.weather.to_lowercase().as_str() {
        "rain" => scenes::city::weather::Weather::Rain,
        "snow" => scenes::city::weather::Weather::Snow,
        _ => scenes::city::weather::Weather::Clear,
    };

    // Build and configure the city scene
    let mut city = scenes::city::CityScene::new();
    city.weather = weather;
    city.simulation_config = config.simulation.clone();
    city.monolith_sign_text = if config.monolith.custom_text.is_empty() {
        let distro = city.distro.to_uppercase();
        format!("{} CORP", distro)
    } else {
        config.monolith.custom_text.clone()
    };
    let mut scene: Box<dyn scene::Scene> = Box::new(city);

    // Run engine
    let mut engine = engine::Engine::new(&config);
    engine.run(scene.as_mut())?;

    Ok(())
}
