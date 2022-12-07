use crossterm::{
    cursor::{self, Hide, Show},
    event::{self, Event, KeyCode},
    execute, queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use genkivision::{Song, Voting};
use std::io::{stdout, Stdout, Write};

const SONG_PRINT_COLUMN_SIZE: u16 = 3;

fn main() -> Result<()> {
    let mut stdout = stdout();

    // Reset & prepare the terminal
    terminal::enable_raw_mode()?;

    execute!(
        stdout,
        EnterAlternateScreen,
        Hide,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    //
    let mut songs_vec: Vec<Song> = Vec::new();

    songs_vec.push(Song::new(
        String::from("Jackie Down the Line"),
        String::from("Fontaines D.C."),
        String::from("Jorge"),
    ));
    songs_vec.push(Song::new(
        String::from("Somewhat Damaged"),
        String::from("Nine Inch Nails"),
        String::from("Selu"),
    ));

    //
    let mut selected_song: usize = 0;
    let mut cursor_position: u16 = 0;

    // Game loop
    'gameloop: loop {
        //
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        //
        for song in songs_vec.iter() {
            print_song(&mut stdout, song)?;
        }
        print_cursor(&mut stdout, cursor_position)?;
        stdout.flush()?;

        // Input bloking
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    break 'gameloop;
                }
                KeyCode::Down => {
                    selected_song += 1;
                    cursor_position += SONG_PRINT_COLUMN_SIZE;
                }
                KeyCode::Up => {
                    selected_song -= 1;
                    cursor_position -= SONG_PRINT_COLUMN_SIZE;
                }
                KeyCode::Char('+') => {
                    songs_vec[selected_song].up_vote();
                }
                KeyCode::Char('-') => {
                    songs_vec[selected_song].down_vote();
                }
                _ => {}
            }
        }
    }

    // Cleanup
    execute!(stdout, LeaveAlternateScreen, Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn print_song(stdout: &mut Stdout, song: &Song) -> Result<()> {
    queue!(
        stdout,
        //
        SetBackgroundColor(Color::White),
        //
        Print("  🎵 "),
        // Song artist
        SetForegroundColor(Color::DarkBlue),
        Print(song.artist.to_string()),
        Print("  "),
        // Song name
        SetForegroundColor(Color::Red),
        SetAttribute(Attribute::Bold),
        Print(song.name.to_string()),
        cursor::MoveToNextLine(1),
        // Votes
        Print("     "),
        Print("🔥".repeat(song.up_votes as usize)),
        Print(" "),
        Print("🤯".repeat(song.down_votes as usize)),
        Print("    "),
        cursor::MoveToNextLine(2),
        // Reset
        ResetColor,
    )
}

fn print_cursor(stdout: &mut Stdout, position: u16) -> Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(0, position),
        SetBackgroundColor(Color::White),
        Print("▶️"),
        //
        ResetColor
    )
}
