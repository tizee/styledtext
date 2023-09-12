use clap::ValueEnum;
use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum CharacterType {
    Letter,
    Digit,
    Greek,
    Other,
}

#[derive(ValueEnum, Debug, Clone)]
#[value(rename_all = "lower")]
pub enum StyledLetter {
    Serif,
    SansSerif,
    Script,
    Fraktur,
    MonoSpace,
    DoubleStruck,
}

use std::error::Error;

#[derive(Debug)]
pub enum LetterTypeError {
    ExceedLengthError(usize),
    InvalidTypeError,
    InvalidStyleError,
    InvalidCodeError(u32),
}

impl Display for LetterTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LetterTypeError::InvalidCodeError(code) => f.write_str(format!("invalid code point {:#04x}", code).as_str()),
            LetterTypeError::ExceedLengthError(index) => f.write_str(format!("{} exceed length", index).as_str()),
            LetterTypeError::InvalidStyleError => f.write_str("invalid style"),
            LetterTypeError::InvalidTypeError => f.write_str("invalid type"),
        }
    }
}

impl Error for LetterTypeError {}

#[derive(ValueEnum, Debug, Clone)]
#[value(rename_all = "lower")]
pub enum LetterStyle {
    Normal,
    Bold,
    Italic,
    BoldItalic,
}

trait TypeStyle {
    fn get_char(
        &self,
        offset: usize,
        style: &LetterStyle,
        uppercase: bool,
    ) -> Result<char, LetterTypeError>;
}

// a field for the number of characters
#[derive(Debug)]
struct SerifType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

macro_rules! impl_type_style {
    ($type_name: ident) => {
        impl TypeStyle for $type_name {
            fn get_char(
                &self,
                offset: usize,
                style: &LetterStyle,
                uppercase: bool,
            ) -> Result<char, LetterTypeError> {
                match *style {
                    LetterStyle::Normal => self.normal(offset, uppercase),
                    LetterStyle::Italic => self.italic(offset, uppercase),
                    LetterStyle::Bold => self.bold(offset, uppercase),
                    LetterStyle::BoldItalic => self.bold_italic(offset, uppercase),
                }
            }
        }
    };
}

impl_type_style!(SerifType);
impl_type_style!(SansSerifType);
impl_type_style!(ScriptType);
impl_type_style!(FrakturType);
impl_type_style!(MonoSpaceType);
impl_type_style!(DoubleStruckType);
impl_type_style!(GreekType);
impl_type_style!(GreekSansSerifType);
impl_type_style!(DigitType);
impl_type_style!(DigitSansSerifType);
impl_type_style!(DigitMonoSpaceType);
impl_type_style!(DigitDoubleStruckType);

impl SerifType {
    fn new() -> Self {
        Self {
            number: 26,
            normal_start: Some((0x41, 0x61)),
            bold_start: Some((0x1D400, 0x1D41A)),
            italic_start: Some((0x1D434, 0x1D44E)),
            bold_italic_start: Some((0x1D468, 0x1D482)),
        }
    }

    fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.normal_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    // lowercase letters
    // serif
    // italic
    // h 0x210E ℎ
    fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.italic_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                if offset == 7 {
                    return Ok('\u{210E}');
                }
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_italic_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
struct SansSerifType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

impl SansSerifType {
    fn new() -> Self {
        Self {
            number: 26,
            normal_start: Some((0x1D5A0, 0x1D5BA)),
            bold_start: Some((0x1D5D4, 0x1D5EE)),
            italic_start: Some((0x1D608, 0x1D622)),
            bold_italic_start: Some((0x1D63C, 0x1D656)),
        }
    }

    fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.normal_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.italic_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_italic_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
struct ScriptType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value.
    // If the style is not supported, then it is None.
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

impl ScriptType {
    pub fn new() -> Self {
        Self {
            number: 26,
            normal_start: Some((0x1D49C, 0x1D4B6)),
            bold_start: Some((0x1D4D0, 0x1D4EA)),
            italic_start: None,
            bold_italic_start: None,
        }
    }

    // # abnormal script unicodes
    // Uppercase normal
    // 0x212C ℬ B
    // 0x2130 ℰ E
    // 0x2131 ℱ F
    // 0x210B ℋ H
    // 0x2110 ℐ J
    // 0x2112 ℒ L
    // 0x2133 ℳ M
    // 0x211B ℛ R

    // script
    // normal
    // e 0x212F ℯ
    // g 0x210A ℊ
    // o 0x2134 ℴ
    pub fn get_normal_corner_case(uppercase: bool) -> (Vec<usize>, Vec<char>) {
        let uppercase_corner_cases: Vec<usize> = vec![1, 4, 5, 7, 8, 11, 12, 17];
        let lowercase_corner_cases: Vec<usize> = vec![4, 6, 14];
        let uppercase_chs: Vec<char> = vec![
            '\u{212C}', // ℬ B
            '\u{2130}', // ℰ E
            '\u{2131}', // ℱ F
            '\u{210B}', // ℋ H
            '\u{2110}', // ℐ I
            '\u{2112}', // ℒ L
            '\u{2133}', // ℳ M
            '\u{211B}', // ℛ R
        ];
        let lowercase_chs: Vec<char> = vec![
            '\u{212F}', // ℯ e
            '\u{210A}', // ℊ g
            '\u{2134}', // ℴ o
        ];
        if uppercase {
            (uppercase_corner_cases, uppercase_chs)
        } else {
            (lowercase_corner_cases, lowercase_chs)
        }
    }

    pub fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        // input ch -> offset/type/style/uppercase
        if let Some((uppercase_start, lowercase_start)) = self.normal_start {
            if uppercase {
                let (uppercase_corner_cases, uppercase_chs) =
                    Self::get_normal_corner_case(uppercase);
                if let Ok(index) = uppercase_corner_cases.binary_search(&offset) {
                    return Ok(uppercase_chs[index]);
                }
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                let (lowercase_corner_cases, lowercase_chs) =
                    Self::get_normal_corner_case(uppercase);
                if let Ok(index) = lowercase_corner_cases.binary_search(&offset) {
                    return Ok(lowercase_chs[index]);
                }
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }
        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
struct FrakturType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

impl FrakturType {
    pub fn new() -> Self {
        Self {
            number: 26,
            normal_start: Some((0x1D504, 0x1D51E)),
            bold_start: Some((0x1D56C, 0x1D586)),
            italic_start: None,
            bold_italic_start: None,
        }
    }

    pub fn get_normal_corner_case() -> (Vec<usize>, Vec<char>) {
        let uppercase_corner_cases: Vec<usize> = vec![2, 7, 8, 17, 25];
        let uppercase_chs: Vec<char> = vec![
            '\u{212D}', // C  ℭ
            '\u{210C}', // H  ℌ
            '\u{2111}', // I  ℑ
            '\u{211C}', // R ℜ
            '\u{2128}', // Z  ℨ
        ];
        return (uppercase_corner_cases, uppercase_chs);
    }

    // # abnormal Fraktur
    // normal
    // C 0x212D ℭ
    // H 0x210C ℌ
    // I 0x2111 ℑ
    // R 0x211C ℜ
    // Z 0x2128 ℨ
    pub fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.normal_start {
            if uppercase {
                let (uppercase_corner_cases, uppercase_chs) = Self::get_normal_corner_case();
                if let Ok(index) = uppercase_corner_cases.binary_search(&offset) {
                    return Ok(uppercase_chs[index]);
                }
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
struct MonoSpaceType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

impl MonoSpaceType {
    pub fn new() -> Self {
        Self {
            number: 26,
            normal_start: Some((0x1D670, 0x1D68A)),
            bold_start: None,
            italic_start: None,
            bold_italic_start: None,
        }
    }

    pub fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.normal_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
struct DoubleStruckType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

impl DoubleStruckType {
    pub fn new() -> Self {
        Self {
            number: 26,
            normal_start: None,
            bold_start: Some((0x1D538, 0x1D552)),
            italic_start: None,
            bold_italic_start: None,
        }
    }

    pub fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }
    // # abnormal double-struck
    // bold
    // C 0x2102 ℂ
    // H 0x210D ℍ
    // N 0x2115 ℕ
    // P 0x2119 ℙ
    // Q 0x211A ℚ
    // R 0x211D ℝ
    // Z 0x2124 ℤ
    pub fn get_bold_corner_case() -> (Vec<usize>, Vec<char>) {
        let indices = vec![2,7,13,15,16,17,25];
        let chars = vec![
'\u{2102}',  // C 0x2102 ℂ
'\u{210D}',  // H 0x210D ℍ
'\u{2115}',  // N 0x2115 ℕ
'\u{2119}',  // P 0x2119 ℙ
'\u{211A}',  // Q 0x211A ℚ
'\u{211D}',  // R 0x211D ℝ
'\u{2124}',  // Z 0x2124 ℤ
        ];
        (indices, chars)
    }

    pub fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_start {
            if uppercase {
                let (indices, chars) = Self::get_bold_corner_case();
                if let Ok(idx) = indices.binary_search(&offset) {
                    return Ok(chars[idx]);
                }
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
struct GreekType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

impl GreekType {
    pub fn new() -> Self {
        Self {
            number: 26,
            normal_start: Some((0x391, 0x3B1)),
            bold_start: Some((0x1D6A8, 0x1D6C2)),
            italic_start: Some((0x1D6E2, 0x1D6FC)),
            bold_italic_start: Some((0x1D71C, 0x1D736)),
        }
    }

    pub fn get_normal_corner_case(uppercase: bool) -> (Vec<usize>, Vec<char>) {
        let uppercase_chs: Vec<char> = vec![
           '\u{3F4}', // ϴ 17
           '\u{2207}', // ∇  25
        ];
        let uppercase_corner_cases: Vec<usize> = vec![17,25];
        let lowercase_chs: Vec<char> = vec![
           '\u{2202}',// ∂ 25
           '\u{3F5}', // ϵ 26
           '\u{3D1}', // ϑ 27
           '\u{3F0}', //  ϰ 28
           '\u{3D5}', //  ϕ 29
           '\u{3F1}', //  ϱ 30
           '\u{3D6}', //  ϖ 31
        ];
        let lowercase_corner_cases: Vec<usize> = vec![25,26,27,28,29,30,31];
        if uppercase {
            (uppercase_corner_cases,uppercase_chs)
        }else{
            (lowercase_corner_cases,lowercase_chs)
        }
    }

    pub fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.normal_start {
            if uppercase {
                let (indices,chs) = Self::get_normal_corner_case(uppercase);
                if let Ok(idx) = indices.binary_search(&offset)  {
                    return Ok(chs[idx]);
                }
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                let (indices,chs) = Self::get_normal_corner_case(uppercase);
                if let Ok(idx) = indices.binary_search(&offset)  {
                    return Ok(chs[idx]);
                }
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.italic_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_italic_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
struct GreekSansSerifType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

impl GreekSansSerifType {
    pub fn new() -> Self {
        Self {
            number: 26,
            normal_start: None,
            bold_start: Some((0x1D756, 0x1D770)),
            italic_start: None,
            bold_italic_start: Some((0x1D790, 0x1D7AA)),
        }
    }

    pub fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_italic_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
struct DigitType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

impl DigitType {
    pub fn new() -> Self {
        Self {
            number: 10,
            normal_start: Some((0x30, 0x30)),
            bold_start: Some((0x1D7CE, 0x1D7CE)),
            italic_start: None,
            bold_italic_start: None,
        }
    }

    pub fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.normal_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.bold_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
struct DigitDoubleStruckType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

impl DigitDoubleStruckType {
    pub fn new() -> Self {
        Self {
            number: 10,
            normal_start: Some((0x1D7D8, 0x1D7D8)),
            bold_start: None,
            italic_start: None,
            bold_italic_start: None,
        }
    }

    pub fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.normal_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
struct DigitSansSerifType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

impl DigitSansSerifType {
    pub fn new() -> Self {
        Self {
            number: 10,
            normal_start: Some((0x1D7E2, 0x1D7E2)),
            bold_start: Some((0x1D7EC, 0x1D7EC)),
            italic_start: None,
            bold_italic_start: None,
        }
    }

    pub fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.normal_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }
        if let Some((uppercase_start, lowercase_start)) = self.bold_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
struct DigitMonoSpaceType {
    number: usize,
    // (uppercase, lowercase)
    // If there is no cases, then we fill them with the same value
    normal_start: Option<(u32, u32)>,
    bold_start: Option<(u32, u32)>,
    italic_start: Option<(u32, u32)>,
    bold_italic_start: Option<(u32, u32)>,
}

impl DigitMonoSpaceType {
    pub fn new() -> Self {
        Self {
            number: 10,
            normal_start: Some((0x1D7F6, 0x1D7F6)),
            bold_start: None,
            italic_start: None,
            bold_italic_start: None,
        }
    }

    pub fn normal(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        if let Some((uppercase_start, lowercase_start)) = self.normal_start {
            if uppercase {
                return Ok(char::from_u32(uppercase_start + offset as u32).unwrap());
            } else {
                return Ok(char::from_u32(lowercase_start + offset as u32).unwrap());
            }
        }
        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }

    pub fn bold_italic(&self, offset: usize, uppercase: bool) -> Result<char, LetterTypeError> {
        if offset >= self.number {
            // eprintln!("Use offset {:?} exceeds the length of SerifType {}", offset, self.number);
            return Err(LetterTypeError::ExceedLengthError(offset));
        }

        Err(LetterTypeError::InvalidStyleError)
    }
}

#[derive(Debug)]
pub struct LetterInfo {
    pub(crate) offset: usize,
    pub(crate) uppercase: bool,
    pub(crate) letter_type: StyledLetter,
    pub(crate) letter_style: LetterStyle,
    pub(crate) character_type: CharacterType,
}

pub enum CharacterInfo {
    Letter(LetterInfo),
    Other(char),
}

impl CharacterInfo {
    pub fn get_letter_info(ch: char) -> CharacterInfo {
        let val = ch as u32;
        match ch {
            // serif
            '\u{0041}'..='\u{005A}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x41) as usize,
                true,
                StyledLetter::Serif,
                LetterStyle::Normal,
                CharacterType::Letter,
            )), // normal
            '\u{1D400}'..='\u{1D419}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D400) as usize,
                true,
                StyledLetter::Serif,
                LetterStyle::Bold,
                CharacterType::Letter,
            )), // bold
            '\u{1D434}'..='\u{1D44D}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D434) as usize,
                true,
                StyledLetter::Serif,
                LetterStyle::Italic,
                CharacterType::Letter,
            )), // italic
            '\u{1D468}'..='\u{1D481}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D468) as usize,
                true,
                StyledLetter::Serif,
                LetterStyle::BoldItalic,
                CharacterType::Letter,
            )), // bold italic
            // sans-serif
            '\u{1D5A0}'..='\u{1D5B9}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D5A0) as usize,
                true,
                StyledLetter::SansSerif,
                LetterStyle::Normal,
                CharacterType::Letter,
            )), // normal
            '\u{1D5D4}'..='\u{1D5ED}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D5D4) as usize,
                true,
                StyledLetter::SansSerif,
                LetterStyle::Bold,
                CharacterType::Letter,
            )), // bold
            '\u{1D608}'..='\u{1D621}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D608) as usize,
                true,
                StyledLetter::SansSerif,
                LetterStyle::Italic,
                CharacterType::Letter,
            )), // italic
            '\u{1D63C}'..='\u{1D655}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D63C) as usize,
                true,
                StyledLetter::SansSerif,
                LetterStyle::BoldItalic,
                CharacterType::Letter,
            )), // bold italic
            // Script
            '\u{1D49C}'..='\u{1D4B5}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D49C) as usize,
                true,
                StyledLetter::Script,
                LetterStyle::Normal,
                CharacterType::Letter,
            )), // normal
            '\u{1D4D0}'..='\u{1D4E9}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D4D0) as usize,
                true,
                StyledLetter::Script,
                LetterStyle::Bold,
                CharacterType::Letter,
            )), // bold
            // Fraktur
            '\u{1D504}'..='\u{1D51D}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D504) as usize,
                true,
                StyledLetter::Fraktur,
                LetterStyle::Normal,
                CharacterType::Letter,
            )), // normal
            '\u{1D56C}'..='\u{1D585}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D56C) as usize,
                true,
                StyledLetter::Fraktur,
                LetterStyle::Bold,
                CharacterType::Letter,
            )), // bold
            // MonoSpace
            '\u{1D670}'..='\u{1D689}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D670) as usize,
                true,
                StyledLetter::MonoSpace,
                LetterStyle::Normal,
                CharacterType::Letter,
            )), // normal
            // DoubleStruck
            '\u{1D538}'..='\u{1D551}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D538) as usize,
                true,
                StyledLetter::DoubleStruck,
                LetterStyle::Bold,
                CharacterType::Letter,
            )), // bold
            // Greek
            '\u{0391}'..='\u{03AA}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x0391) as usize,
                true,
                StyledLetter::Serif,
                LetterStyle::Normal,
                CharacterType::Greek,
            )), // normal
            '\u{1D6A8}'..='\u{1D6C1}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D6A8) as usize,
                true,
                StyledLetter::Serif,
                LetterStyle::Bold,
                CharacterType::Greek,
            )), // bold
            '\u{1D6E2}'..='\u{1D6FB}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D6E2) as usize,
                true,
                StyledLetter::Serif,
                LetterStyle::Italic,
                CharacterType::Greek,
            )), // italic
            '\u{1D71C}'..='\u{1D735}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D71C) as usize,
                true,
                StyledLetter::Serif,
                LetterStyle::BoldItalic,
                CharacterType::Greek,
            )), // bold-italic
            '\u{1D756}'..='\u{1D76F}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D756) as usize,
                true,
                StyledLetter::SansSerif,
                LetterStyle::Bold,
                CharacterType::Greek,
            )), // sans-serif bold
            '\u{1D790}'..='\u{1D7A9}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D790) as usize,
                true,
                StyledLetter::SansSerif,
                LetterStyle::BoldItalic,
                CharacterType::Greek,
            )), // sans-serif bold italic
            // ------ Lowercase ------
            // Serif
            '\u{0061}'..='\u{007A}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x61) as usize,
                false,
                StyledLetter::Serif,
                LetterStyle::Normal,
                CharacterType::Letter,
            )), // normal
            '\u{1D41A}'..='\u{1D433}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D41A) as usize,
                false,
                StyledLetter::Serif,
                LetterStyle::Bold,
                CharacterType::Letter,
            )), // bold
            '\u{1D44E}'..='\u{1D467}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D44E) as usize,
                false,
                StyledLetter::Serif,
                LetterStyle::Italic,
                CharacterType::Letter,
            )), // italic
            '\u{1D482}'..='\u{1D49B}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D482) as usize,
                false,
                StyledLetter::Serif,
                LetterStyle::BoldItalic,
                CharacterType::Letter,
            )), // bold italic
            // sans-serif
            '\u{1D5BA}'..='\u{1D5D3}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D5BA) as usize,
                false,
                StyledLetter::SansSerif,
                LetterStyle::Normal,
                CharacterType::Letter,
            )), // normal
            '\u{1D5EE}'..='\u{1D607}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D5EE) as usize,
                false,
                StyledLetter::SansSerif,
                LetterStyle::Bold,
                CharacterType::Letter,
            )), // bold
            '\u{1D622}'..='\u{1D63B}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D622) as usize,
                false,
                StyledLetter::SansSerif,
                LetterStyle::Italic,
                CharacterType::Letter,
            )), // italic
            '\u{1D656}'..='\u{1D66F}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D656) as usize,
                false,
                StyledLetter::SansSerif,
                LetterStyle::BoldItalic,
                CharacterType::Letter,
            )), // bold italic
            // Script
            '\u{1D4B6}'..='\u{1D4CF}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D4B6) as usize,
                false,
                StyledLetter::Script,
                LetterStyle::Normal,
                CharacterType::Letter,
            )), // normal
            '\u{1D4EA}'..='\u{1D503}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D4EA) as usize,
                false,
                StyledLetter::Script,
                LetterStyle::Bold,
                CharacterType::Letter,
            )), // bold
            // Fraktur
            '\u{1D51E}'..='\u{1D537}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D51E) as usize,
                false,
                StyledLetter::Fraktur,
                LetterStyle::Normal,
                CharacterType::Letter,
            )), // normal
            '\u{1D586}'..='\u{1D59F}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D586) as usize,
                false,
                StyledLetter::Fraktur,
                LetterStyle::Bold,
                CharacterType::Letter,
            )), // bold
            // MonoSpace
            '\u{1D68A}'..='\u{1D6A3}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D68A) as usize,
                false,
                StyledLetter::MonoSpace,
                LetterStyle::Normal,
                CharacterType::Letter,
            )), // normal
            // DoubleStruck
            '\u{1D552}'..='\u{1D56B}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D552) as usize,
                false,
                StyledLetter::DoubleStruck,
                LetterStyle::Bold,
                CharacterType::Letter,
            )), // bold
            // Greek
            '\u{03B1}'..='\u{03D0}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x03B1) as usize,
                false,
                StyledLetter::Serif,
                LetterStyle::Normal,
                CharacterType::Greek,
            )), // normal
            '\u{1D6C2}'..='\u{1D6E1}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D6C2) as usize,
                false,
                StyledLetter::Serif,
                LetterStyle::Bold,
                CharacterType::Greek,
            )), // bold
            '\u{1D6FC}'..='\u{1D71B}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D6FC) as usize,
                false,
                StyledLetter::Serif,
                LetterStyle::Italic,
                CharacterType::Greek,
            )), // italic
            '\u{1D736}'..='\u{1D755}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D736) as usize,
                false,
                StyledLetter::Serif,
                LetterStyle::BoldItalic,
                CharacterType::Greek,
            )), // bold-italic
            '\u{1D770}'..='\u{1D78F}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D770) as usize,
                false,
                StyledLetter::SansSerif,
                LetterStyle::Bold,
                CharacterType::Greek,
            )), // sans-serif bold
            '\u{1D7AA}'..='\u{1D7C9}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D7AA) as usize,
                false,
                StyledLetter::SansSerif,
                LetterStyle::BoldItalic,
                CharacterType::Greek,
            )), // sans-serif bold italic
            // ------ Lowercase ------
            // Digits
            '\u{0030}'..='\u{0039}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x0030) as usize,
                false,
                StyledLetter::Serif,
                LetterStyle::Normal,
                CharacterType::Digit,
            )), // normal
            '\u{1D7CE}'..='\u{1D7D7}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D7CE) as usize,
                false,
                StyledLetter::Serif,
                LetterStyle::Bold,
                CharacterType::Digit,
            )), // bold
            '\u{1D7D8}'..='\u{1D7E1}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D7D8) as usize,
                false,
                StyledLetter::DoubleStruck,
                LetterStyle::Normal,
                CharacterType::Digit,
            )), // normal
            '\u{1D7E2}'..='\u{1D7EB}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D7E2) as usize,
                false,
                StyledLetter::SansSerif,
                LetterStyle::Normal,
                CharacterType::Digit,
            )), // normal
            '\u{1D7EC}'..='\u{1D7F5}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D7EC) as usize,
                false,
                StyledLetter::SansSerif,
                LetterStyle::Bold,
                CharacterType::Digit,
            )), // bold
            '\u{1D7F6}'..='\u{1D7FF}' => CharacterInfo::Letter(LetterInfo::new(
                (val - 0x1D7F6) as usize,
                false,
                StyledLetter::DoubleStruck,
                LetterStyle::Normal,
                CharacterType::Digit,
            )), // normal
            _ => {
                // corner cases
                // script
                let (uppercase_script_indices, uppercase_script_chs) =
                    ScriptType::get_normal_corner_case(true);
                let (lowercase_script_indices, lowercase_script_chs) =
                    ScriptType::get_normal_corner_case(false);
                if let Ok(index) = uppercase_script_chs.binary_search(&ch) {
                    return CharacterInfo::Letter(LetterInfo::new(
                        uppercase_script_indices[index],
                        true,
                        StyledLetter::Script,
                        LetterStyle::Normal,
                        CharacterType::Letter,
                    ));
                }
                if let Ok(index) = lowercase_script_chs.binary_search(&ch) {
                    return CharacterInfo::Letter(LetterInfo::new(
                        lowercase_script_indices[index],
                        true,
                        StyledLetter::Script,
                        LetterStyle::Normal,
                        CharacterType::Letter,
                    ));
                }
                // fraktur
                let (uppercase_fraktur_indices, uppercase_fraktur_chs) =
                    FrakturType::get_normal_corner_case();
                if let Ok(index) = uppercase_fraktur_chs.binary_search(&ch) {
                    return CharacterInfo::Letter(LetterInfo::new(
                        uppercase_fraktur_indices[index],
                        true,
                        StyledLetter::Fraktur,
                        LetterStyle::Normal,
                        CharacterType::Letter,
                    ));
                }
                // serif italic
                if ch == '\u{210E}' {
                    return CharacterInfo::Letter(LetterInfo::new(
                        7,
                        true,
                        StyledLetter::Serif,
                        LetterStyle::Italic,
                        CharacterType::Letter,
                    ));
                }

                return CharacterInfo::Other(ch);
            }
        }
    }
}

impl LetterInfo {
    pub fn new(
        offset: usize,
        uppercase: bool,
        letter_type: StyledLetter,
        letter_style: LetterStyle,
        character_type: CharacterType,
    ) -> Self {
        Self {
            offset,
            uppercase,
            letter_type,
            letter_style,
            character_type,
        }
    }

    pub fn convert(
        &self,
        letter_type: &StyledLetter,
        letter_style: &LetterStyle,
    ) -> Result<char, LetterTypeError> {
        match self.character_type {
            CharacterType::Letter => match *letter_type {
                StyledLetter::Serif => {
                    let serif = SerifType::new();
                    serif.get_char(self.offset, letter_style, self.uppercase)
                }
                StyledLetter::SansSerif => {
                    let sans_serif = SansSerifType::new();
                    sans_serif.get_char(self.offset, letter_style, self.uppercase)
                }
                StyledLetter::Script => {
                    let script = ScriptType::new();
                    script.get_char(self.offset, letter_style, self.uppercase)
                }
                StyledLetter::Fraktur => {
                    let fraktur = FrakturType::new();
                    fraktur.get_char(self.offset, letter_style, self.uppercase)
                }
                StyledLetter::MonoSpace => {
                    let mono = MonoSpaceType::new();
                    mono.get_char(self.offset, letter_style, self.uppercase)
                }
                StyledLetter::DoubleStruck => {
                    let double_struck = DoubleStruckType::new();
                    double_struck.get_char(self.offset, letter_style, self.uppercase)
                }
            },
            CharacterType::Greek => match letter_type {
                StyledLetter::Serif => {
                    let serif = GreekType::new();
                    serif.get_char(self.offset, letter_style, self.uppercase)
                }
                StyledLetter::SansSerif => {
                    let sans_serif = GreekSansSerifType::new();
                    sans_serif.get_char(self.offset, letter_style, self.uppercase)
                }
                StyledLetter::Script => Err(LetterTypeError::InvalidTypeError),
                StyledLetter::Fraktur => Err(LetterTypeError::InvalidTypeError),
                StyledLetter::MonoSpace => Err(LetterTypeError::InvalidTypeError),
                StyledLetter::DoubleStruck => Err(LetterTypeError::InvalidTypeError),
            },
            CharacterType::Digit => match letter_type {
                StyledLetter::Serif => {
                    let serif = DigitType::new();
                    serif.get_char(self.offset, letter_style, self.uppercase)
                }
                StyledLetter::SansSerif => {
                    let sans_serif = DigitSansSerifType::new();
                    sans_serif.get_char(self.offset, letter_style, self.uppercase)
                }
                StyledLetter::Script => Err(LetterTypeError::InvalidTypeError),
                StyledLetter::Fraktur => Err(LetterTypeError::InvalidTypeError),
                StyledLetter::MonoSpace => {
                    let mono = DigitMonoSpaceType::new();
                    mono.get_char(self.offset, letter_style, self.uppercase)
                }
                StyledLetter::DoubleStruck => {
                    let double_struck = DigitDoubleStruckType::new();
                    double_struck.get_char(self.offset, letter_style, self.uppercase)
                }
            },
            CharacterType::Other => Err(LetterTypeError::InvalidTypeError),
        }
    }
}

impl ToString for StyledLetter {
    fn to_string(&self) -> String {
        match *self {
            StyledLetter::Serif => "serif".to_string(),
            StyledLetter::SansSerif => "sans_serif".to_string(),
            StyledLetter::Script => "script".to_string(),
            StyledLetter::Fraktur => "fraktur".to_string(),
            StyledLetter::MonoSpace => "mono".to_string(),
            StyledLetter::DoubleStruck => "double_struck".to_string(),
        }
    }
}

impl ToString for LetterStyle {
    fn to_string(&self) -> String {
        match *self {
            LetterStyle::Bold => "bold".to_string(),
            LetterStyle::BoldItalic => "bold_italic".to_string(),
            LetterStyle::Italic => "italic".to_string(),
            LetterStyle::Normal => "normal".to_string(),
        }
    }
}

#[cfg(test)]
mod test_ascii {
    use super::*;
    #[test]
    pub fn test_script_corner_cases() {
        let script_type = ScriptType::new();
        let uppercase_corner_cases: Vec<usize> = vec![1, 4, 5, 7, 9, 11, 12, 17];
        let lowercase_corner_cases: Vec<usize> = vec![4, 6, 14];
        let uppercase_chs: Vec<char> = vec![
            '\u{212C}', // ℬ B
            '\u{2130}', // ℰ E
            '\u{2131}', // ℱ F
            '\u{210B}', // ℋ H
            '\u{2110}', // ℐ J
            '\u{2112}', // ℒ L
            '\u{2133}', // ℳ M
            '\u{211B}', // ℛ R
        ];
        let lowercase_chs: Vec<char> = vec![
            '\u{212F}', // ℯ e
            '\u{210A}', // ℊ g
            '\u{2134}', // ℴ o
        ];

        for (idx, val) in uppercase_corner_cases.iter().enumerate() {
            let res = script_type.normal(*val, true);
            assert_eq!(true, res.is_ok());
            let ch = res.unwrap();
            assert_eq!(uppercase_chs[idx], ch);
        }

        for (idx, val) in lowercase_corner_cases.iter().enumerate() {
            let res = script_type.normal(*val, false);
            assert_eq!(true, res.is_ok());
            let ch = res.unwrap();
            assert_eq!(lowercase_chs[idx], ch);
        }
    }

    #[test]
    pub fn test_fraktur_corner_cases() {
        let fraktur_type = FrakturType::new();
        let uppercase_corner_cases: Vec<usize> = vec![2, 7, 8, 17, 25];
        let uppercase_chs: Vec<char> = vec![
            '\u{212D}', // C  ℭ
            '\u{210C}', // H  ℌ
            '\u{2111}', // I  ℑ
            '\u{211C}', // R ℜ
            '\u{2128}', // Z  ℨ
        ];

        for (idx, val) in uppercase_corner_cases.iter().enumerate() {
            let res = fraktur_type.normal(*val, true);
            assert!(res.is_ok());
            let ch = res.unwrap();
            assert_eq!(uppercase_chs[idx], ch);
        }
    }

    #[test]
    pub fn test_serif_corner_cases() {
        let serif_type = SerifType::new();
        let res = serif_type.italic(7, false);
        assert!(res.is_ok());
        let ch = res.unwrap();
        assert_eq!('\u{210E}', ch);
    }

    #[test]
    pub fn test_character_info() {
        fn test_seq(s: &str) {
            let offsets: Vec<usize> = (0..=25).collect();
            for (idx, ch) in s.chars().enumerate() {
                let res = CharacterInfo::get_letter_info(ch);
                match res {
                    CharacterInfo::Letter(info) => {
                        println!("{} - {:?}", ch, info);
                        assert_eq!(offsets[idx], info.offset);
                    }
                    _ => {
                        panic!();
                    }
                }
            }
        }
        // Serif
        // normal
        test_seq("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        test_seq("abcdefghijklmnopqrstuvwxyz");
        // bold
        test_seq("𝐀𝐁𝐂𝐃𝐄𝐅𝐆𝐇𝐈𝐉𝐊𝐋𝐌𝐍𝐎𝐏𝐐𝐑𝐒𝐓𝐔𝐕𝐖𝐗𝐘𝐙");
        test_seq("𝐚𝐛𝐜𝐝𝐞𝐟𝐠𝐡𝐢𝐣𝐤𝐥𝐦𝐧𝐨𝐩𝐪𝐫𝐬𝐭𝐮𝐯𝐰𝐱𝐲𝐳");
        // italic
        test_seq("𝐴𝐵𝐶𝐷𝐸𝐹𝐺𝐻𝐼𝐽𝐾𝐿𝑀𝑁𝑂𝑃𝑄𝑅𝑆𝑇𝑈𝑉𝑊𝑋𝑌𝑍");
        test_seq("𝑎𝑏𝑐𝑑𝑒𝑓𝑔ℎ𝑖𝑗𝑘𝑙𝑚𝑛𝑜𝑝𝑞𝑟𝑠𝑡𝑢𝑣𝑤𝑥𝑦𝑧");
        // bold italic
        test_seq("𝑨𝑩𝑪𝑫𝑬𝑭𝑮𝑯𝑰𝑱𝑲𝑳𝑴𝑵𝑶𝑷𝑸𝑹𝑺𝑻𝑼𝑽𝑾𝑿𝒀𝒁");
        test_seq("𝒂𝒃𝒄𝒅𝒆𝒇𝒈𝒉𝒊𝒋𝒌𝒍𝒎𝒏𝒐𝒑𝒒𝒓𝒔𝒕𝒖𝒗𝒘𝒙𝒚𝒛");
        // SansSerif
        // normal
        test_seq("𝖠𝖡𝖢𝖣𝖤𝖥𝖦𝖧𝖨𝖩𝖪𝖫𝖬𝖭𝖮𝖯𝖰𝖱𝖲𝖳𝖴𝖵𝖶𝖷𝖸𝖹");
        test_seq("𝖺𝖻𝖼𝖽𝖾𝖿𝗀𝗁𝗂𝗃𝗄𝗅𝗆𝗇𝗈𝗉𝗊𝗋𝗌𝗍𝗎𝗏𝗐𝗑𝗒𝗓");
        // bold
        test_seq("𝗔𝗕𝗖𝗗𝗘𝗙𝗚𝗛𝗜𝗝𝗞𝗟𝗠𝗡𝗢𝗣𝗤𝗥𝗦𝗧𝗨𝗩𝗪𝗫𝗬𝗭");
        test_seq("𝗮𝗯𝗰𝗱𝗲𝗳𝗴𝗵𝗶𝗷𝗸𝗹𝗺𝗻𝗼𝗽𝗾𝗿𝘀𝘁𝘂𝘃𝘄𝘅𝘆𝘇");
        // italic
        test_seq("𝘈𝘉𝘊𝘋𝘌𝘍𝘎𝘏𝘐𝘑𝘒𝘓𝘔𝘕𝘖𝘗𝘘𝘙𝘚𝘛𝘜𝘝𝘞𝘟𝘠𝘡");
        test_seq("𝘢𝘣𝘤𝘥𝘦𝘧𝘨𝘩𝘪𝘫𝘬𝘭𝘮𝘯𝘰𝘱𝘲𝘳𝘴𝘵𝘶𝘷𝘸𝘹𝘺𝘻");
        // bold italic
        test_seq("𝘼𝘽𝘾𝘿𝙀𝙁𝙂𝙃𝙄𝙅𝙆𝙇𝙈𝙉𝙊𝙋𝙌𝙍𝙎𝙏𝙐𝙑𝙒𝙓𝙔𝙕");
        test_seq("𝙖𝙗𝙘𝙙𝙚𝙛𝙜𝙝𝙞𝙟𝙠𝙡𝙢𝙣𝙤𝙥𝙦𝙧𝙨𝙩𝙪𝙫𝙬𝙭𝙮𝙯");
        // Script
        // normal
        test_seq("𝒜ℬ𝒞𝒟ℰℱ𝒢ℋℐ𝒥𝒦ℒℳ𝒩𝒪𝒫𝒬ℛ𝒮𝒯𝒰𝒱𝒲𝒳𝒴𝒵");
        test_seq("𝒶𝒷𝒸𝒹ℯ𝒻ℊ𝒽𝒾𝒿𝓀𝓁𝓂𝓃ℴ𝓅𝓆𝓇𝓈𝓉𝓊𝓋𝓌𝓍𝓎𝓏");
        // bold
        test_seq("𝓐𝓑𝓒𝓓𝓔𝓕𝓖𝓗𝓘𝓙𝓚𝓛𝓜𝓝𝓞𝓟𝓠𝓡𝓢𝓣𝓤𝓥𝓦𝓧𝓨𝓩");
        test_seq("𝓪𝓫𝓬𝓭𝓮𝓯𝓰𝓱𝓲𝓳𝓴𝓵𝓶𝓷𝓸𝓹𝓺𝓻𝓼𝓽𝓾𝓿𝔀𝔁𝔂𝔃");
        // Fraktur
        // normal
        test_seq("𝔞𝔟𝔠𝔡𝔢𝔣𝔤𝔥𝔦𝔧𝔨𝔩𝔪𝔫𝔬𝔭𝔮𝔯𝔰𝔱𝔲𝔳𝔴𝔵𝔶𝔷");
        test_seq("𝔄𝔅ℭ𝔇𝔈𝔉𝔊ℌℑ𝔍𝔎𝔏𝔐𝔑𝔒𝔓𝔔ℜ𝔖𝔗𝔘𝔙𝔚𝔛𝔜ℨ");
        // bold
        test_seq("𝕬𝕭𝕮𝕯𝕰𝕱𝕲𝕳𝕴𝕵𝕶𝕷𝕸𝕹𝕺𝕻𝕼𝕽𝕾𝕿𝖀𝖁𝖂𝖃𝖄𝖅");
        test_seq("𝖆𝖇𝖈𝖉𝖊𝖋𝖌𝖍𝖎𝖏𝖐𝖑𝖒𝖓𝖔𝖕𝖖𝖗𝖘𝖙𝖚𝖛𝖜𝖝𝖞𝖟");
        // MonoSpace
        test_seq("𝚊𝚋𝚌𝚍𝚎𝚏𝚐𝚑𝚒𝚓𝚔𝚕𝚖𝚗𝚘𝚙𝚚𝚛𝚜𝚝𝚞𝚟𝚠𝚡𝚢𝚣");
        test_seq("𝙰𝙱𝙲𝙳𝙴𝙵𝙶𝙷𝙸𝙹𝙺𝙻𝙼𝙽𝙾𝙿𝚀𝚁𝚂𝚃𝚄𝚅𝚆𝚇𝚈𝚉");
        // DoubleStruck
        test_seq("𝔸𝔹ℂ𝔻𝔼𝔽𝔾ℍ𝕀𝕁𝕂𝕃𝕄ℕ𝕆ℙℚℝ𝕊𝕋𝕌𝕍𝕎𝕏𝕐ℤ");
        test_seq("𝕒𝕓𝕔𝕕𝕖𝕗𝕘𝕙𝕚𝕛𝕜𝕝𝕞𝕟𝕠𝕡𝕢𝕣𝕤𝕥𝕦𝕧𝕨𝕩𝕪𝕫");
    }
}
