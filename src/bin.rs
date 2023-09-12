use clap::Parser;
use styledtextlib::{convert, LetterStyle, StyledLetter};

#[derive(Parser)]
#[command(about, version, long_about = None)]
struct AppArgs {
    text: String,

    /// turn ASCII letters into styled letters
    #[arg(
        value_enum,
        long,
        conflicts_with = "ascii",
        requires = "text",
        default_value = "monospace"
    )]
    letter_type: Option<StyledLetter>,

    #[arg(
        value_enum,
        long,
        conflicts_with = "ascii",
        requires = "text",
        default_value = "normal"
    )]
    letter_style: Option<LetterStyle>,

    /// convert with randomly types and styles
    #[arg(long, conflicts_with = "ascii", conflicts_with = "letter_type")]
    random: bool,

    /// convert text randomly within given types
    #[arg(
        long,
        conflicts_with = "ascii",
        conflicts_with = "letter_type",
        requires = "random"
    )]
    exclude_types: Option<Vec<StyledLetter>>,

    /// convert text randomly within given styles
    #[arg(
        long,
        conflicts_with = "ascii",
        conflicts_with = "letter_type",
        requires = "random"
    )]
    exclude_styles: Option<Vec<LetterStyle>>,

    /// turn styled letters to ASCII letters
    #[arg(long)]
    ascii: bool,
}

fn main() {
    let args = AppArgs::parse();

    let input: String = args.text;
    let letter_type = args.letter_type.unwrap();
    let letter_style = args.letter_style.unwrap();
    if !input.is_empty() {
        let mut res: String = String::with_capacity(input.len());
        for ch in input.chars() {
            match convert(ch, &letter_type, &letter_style) {
                Ok(char) => {
                    res.push(char);
                }
                Err(e) => {
                    eprintln!("Error: {} for {} using {:?}-{:?}", e, ch, letter_type, letter_style);
                }
            }
        }
        println!("{}", res);
    }
}
