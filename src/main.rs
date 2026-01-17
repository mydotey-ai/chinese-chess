use chinese_chess::board::coordinate::Coordinate;
use chinese_chess::game::Game;
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
}

fn main() {
    let cli = Cli::parse();
    let mut game = Game::new();

    match &cli.command {
        Some(Commands::New) => {
            println!("{}", "New Chinese Chess (Xiangqi) game started!".green());
            println!(
                "{}: {}",
                "Current Turn".blue(),
                format!("{:?}", game.current_turn()).red()
            );
        }
        Some(Commands::Move { from, to }) => {
            if let (Some(from_coord), Some(to_coord)) = parse_coordinates(from, to) {
                if game.make_move(from_coord, to_coord) {
                    println!("{}", "Move successful!".green());
                    println!(
                        "{}: {}",
                        "Current Turn".blue(),
                        format!("{:?}", game.current_turn()).red()
                    );
                } else {
                    println!("{}", "Invalid move!".red());
                }
            } else {
                println!("{}", "Invalid coordinate format! Use x,y format.".red());
            }
        }
        None => {
            println!("{}", "Chinese Chess (Xiangqi)".bold().blue());
            println!("{}", "Use --help for available commands.".yellow());
        }
    }
}

fn parse_coordinates(from: &str, to: &str) -> (Option<Coordinate>, Option<Coordinate>) {
    let parse = |s: &str| {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 2 {
            let x: usize = parts[0].parse().ok()?;
            let y: usize = parts[1].parse().ok()?;
            Some(Coordinate::new(x, y))
        } else {
            None
        }
    };

    (parse(from), parse(to))
}
