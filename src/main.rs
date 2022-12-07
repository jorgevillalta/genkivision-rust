use crossterm::{
    cursor, execute, queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal, Result,
};
use genkivision::{Song, Voting};
use std::io::{stdout, Stdout, Write};

fn main() -> Result<()> {
    let mut stdout = stdout();

    // Reset & prepare the terminal
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    //
    let mut jor_song = Song::new(
        String::from("Jackie Down the Line"),
        String::from("Fontaines D.C."),
        String::from("Jorge"),
    );
    let mut selu_song = Song::new(
        String::from("Somewhat Damaged"),
        String::from("Nine Inch Nails"),
        String::from("Selu"),
    );

    //
    jor_song.up_vote();
    jor_song.up_vote();
    jor_song.down_vote();
    selu_song.up_vote();
    selu_song.down_vote();
    selu_song.down_vote();
    selu_song.down_vote();

    print_song(&mut stdout, &jor_song)?;
    print_song(&mut stdout, &selu_song)?;
    stdout.flush()?;

    Ok(())
}

fn print_song(stdout: &mut Stdout, song: &Song) -> Result<()> {
    queue!(
        stdout,
        //
        SetBackgroundColor(Color::White),
        //
        Print(" 🎵 "),
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
        Print("    "),
        Print("🔥".repeat(song.up_votes as usize)),
        Print(" "),
        Print("🤯".repeat(song.down_votes as usize)),
        Print("    "),
        cursor::MoveToNextLine(2),
        // Reset
        ResetColor,
    )
}
