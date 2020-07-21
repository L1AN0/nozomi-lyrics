use dialoguer::Select;
use lrc::TimeTag;
use mpris::PlayerFinder;
use nozomi_player::PlayingMusic;
use std::io::Write;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt()]
struct Cli {
    #[structopt(long)]
    prompt: bool,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("LOG_LEVEL"))
        .init();
    let args: Cli = Cli::from_args();

    let player = PlayerFinder::new()
        .expect("could not connect to D-Bus")
        .find_active()
        .expect("could not find any player");

    let mut current_music = PlayingMusic {
        title: "".to_string(),
    };

    let mut lyrics: Option<lrc::Lyrics> = None;
    let mut tlyrics: Option<lrc::Lyrics> = None;

    loop {
        let metadata = player
            .get_metadata()
            .expect("Could not get metadata for player");
        if current_music.title == metadata.title().unwrap() {
            if let Some(lyrics) = &lyrics {
                print!("\x1B[2J\x1B[1;1H");
                std::io::stdout().flush().unwrap();
                let current_position = player.get_position_in_microseconds().unwrap() / 1000;
                tracing::debug!("current position: {}", current_position);
                let index = lyrics.find_timed_line_index(TimeTag::new(current_position as i64));
                if let Some(index) = index {
                    if let Some(line) = lyrics.get_timed_lines().get(index) {
                        println!("{}", line.1);
                    }
                }
                if let Some(tlyrics) = &tlyrics {
                    let index =
                        tlyrics.find_timed_line_index(TimeTag::new(current_position as i64));
                    if let Some(index) = index {
                        if let Some(line) = tlyrics.get_timed_lines().get(index) {
                            println!("{}", line.1);
                        }
                    }
                }
            } else {
                let search_results = current_music.retrieve_163_search_results();
                let selection = if args.prompt {
                    let selection_idx = Select::new()
                        .with_prompt("pick search result")
                        .default(0)
                        .items(&search_results.result.songs)
                        .interact()
                        .unwrap();
                    &search_results.result.songs[selection_idx]
                } else {
                    &search_results.result.songs[0]
                };
                let retrieved_lyrics = selection.retrieve_163_lyrics();
                lyrics = retrieved_lyrics.lrc.to_lyrics();
                tlyrics = retrieved_lyrics.tlyric.to_lyrics();
                tracing::debug!("{:?}", lyrics);
                tracing::debug!("{:?}", lyrics.as_ref().unwrap().get_timed_lines());
            }
        } else {
            tracing::debug!("metadata {:#?}", metadata);
            current_music.title = metadata.title().unwrap().to_owned();
            lyrics = None;
            tlyrics = None;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
}
