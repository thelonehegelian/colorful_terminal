use ansi_term::Colour;

fn main() {
    println!(
        "This is in red: {}{}",
        Colour::Red.paint("a red string"),
        Colour::Yellow.italic().paint(" and it is in Italic"),
    );
}
