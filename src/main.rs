use ansi_term::Colour;

fn main() {
    println!(
        "{} {} {}",
        Colour::Red.paint("A red string"),
        Colour::Yellow.italic().paint("and it is in Italic"),
        Colour::Green.paint(", and we have a green string"),
    );
}
