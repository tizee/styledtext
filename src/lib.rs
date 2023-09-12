mod letter;
pub use letter::{StyledLetter, LetterStyle, CharacterInfo, LetterTypeError};

pub fn convert(ch: char, letter_type: &StyledLetter, letter_style: &LetterStyle) -> Result<char, LetterTypeError> {
    let character_info = CharacterInfo::get_letter_info(ch);
    match character_info {
        CharacterInfo::Letter(info) => {
            info.convert(letter_type, letter_style)
        },
        CharacterInfo::Other(not_supported_ch) => {
            Ok(not_supported_ch)
        }
    }
}

