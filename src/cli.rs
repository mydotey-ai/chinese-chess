use chinese_chess::game::GameStateManager;
use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new game
    New,
    /// Make a move (format: from_x,from_y to_x,to_y)
    Move { from: String, to: String },
    /// Undo last move
    Undo,
}

fn main() {
    let cli = Cli::parse();
    let mut manager = GameStateManager::new();

    match &cli.command {
        Some(Commands::New) => {
            manager = GameStateManager::new();
            println!("{}", "New Chinese Chess (Xiangqi) game started!".green());
            println!(
                "{}: {}",
                "Current Turn".blue(),
                format!("{:?}", manager.state.current_turn).red()
            );
        }
        Some(Commands::Move { from, to }) => {
            if let (Some(from_coord), Some(to_coord)) = parse_coordinates(from, to) {
                match manager.make_move(from_coord.0, from_coord.1, to_coord.0, to_coord.1) {
                    Ok(_) => {
                        println!("{}", "Move successful!".green());
                        println!(
                            "{}: {}",
                            "Current Turn".blue(),
                            format!("{:?}", manager.state.current_turn).red()
                        );
                        if manager.state.is_in_check {
                            println!("{}", "Check!".yellow());
                        }
                        if manager.state.is_ended {
                            println!(
                                "{}: {}",
                                "Game Over! Winner".blue(),
                                format!("{:?}", manager.state.winner).red()
                            );
                        }
                    }
                    Err(e) => {
                        println!("{}: {:?}", "Error".red(), e);
                    }
                }
            } else {
                println!("{}", "Invalid coordinate format! Use x,y format.".red());
            }
        }
        Some(Commands::Undo) => match manager.undo_move() {
            Ok(_) => {
                println!("{}", "Undo successful!".green());
                println!(
                    "{}: {}",
                    "Current Turn".blue(),
                    format!("{:?}", manager.state.current_turn).red()
                );
            }
            Err(e) => {
                println!("{}: {:?}", "Error".red(), e);
            }
        },
        None => {
            println!("{}", "Chinese Chess (Xiangqi)".bold().blue());
            println!("{}", "Use --help for available commands.".yellow());
        }
    }
}

fn parse_coordinates(from: &str, to: &str) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let parse = |s: &str| {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 2 {
            let x: usize = parts[0].parse().ok()?;
            let y: usize = parts[1].parse().ok()?;
            Some((x, y))
        } else {
            None
        }
    };

    (parse(from), parse(to))
}
