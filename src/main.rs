mod fetch;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Lyrics CLI")]
#[command(version = "1.0")]
#[command(about = "Fetches the currently playing lyric line", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the current lyric line
    Line,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Line => {
            let player = fetch::fetch_player().await?;
            let lyrics = fetch::fetch_lyrics(player.song_id).await?;
            if let Some(line) = lyrics.current(player.progress) {
                println!("{}", line.content);
            } else {
                println!("No lyric line found for current progress.");
            }
        }
    }

    Ok(())
}
