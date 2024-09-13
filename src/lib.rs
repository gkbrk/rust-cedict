//! cedict is a Rust crate for reading and writing the CC-CEDICT
//! Chinese-English dictionary format. It can be used to implement Chinese
//! dictionaries in Rust. It can also serve as a tool for automating
//! maintenance to the CEDICT project.
//!
//! # Examples
//! ```
//! let line = "你好 你好 [ni3 hao3] /Hello!/Hi!/How are you?/";
//! let parsed = cedict::parse_line(line);
//!
//! if let cedict::Line::Entry(entry) = parsed {
//!    assert_eq!(entry.simplified(), "你好");
//!    assert_eq!(entry.pinyin(), "ni3 hao3");
//!    assert_eq!(entry.definitions().collect::<Vec<_>>(), vec!["Hello!", "Hi!", "How are you?"]);
//! }
//! ```
//!
//! ```
//! use std::fs::File;
//!
//! match File::open("cedict.txt") {
//!     Ok(file) => {
//!         for word in cedict::parse_reader(file) {
//!             if word.definitions().next().unwrap().contains("Hello") {
//!                 println!("{}", word.simplified());
//!             }
//!         }
//!     },
//!     Err(_) => {
//!         println!("Cannot read file");
//!     }
//! }
//!
//! ```

#![deny(unsafe_code)]

use std::option::Option;

/// Used to represent a range of characters in a string.
type Slice = (usize, usize);

/// Represents a single dictionary entry in the CC-CEDICT format.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DictEntry<T> {
    line: T,
    traditional: Slice,
    simplified: Slice,
    pinyin: Slice,
    definitions: Slice,
}

impl std::fmt::Debug for DictEntry<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "DictEntry {{ traditional: `{:?}`, simplified: `{:?}`, pinyin: `{:?}`, definitions: [{:?}] }}",
            self.traditional(),
            self.simplified(),
            self.pinyin(),
            self.definitions().collect::<Vec<_>>().join("~")
        )
    }
}

impl<T: AsRef<str>> DictEntry<T> {
    /// Returns the traditional characters of the entry.
    /// 
    /// # Examples
    /// ```
    /// let line = "睡覺 睡觉 [shui4 jiao4] /to go to bed/to sleep/";
    /// let parsed = cedict::parse_dict_entry(line).unwrap();
    /// 
    /// assert_eq!(parsed.traditional(), "睡覺");
    /// ```
    pub fn traditional(&self) -> &str {
        &self.line.as_ref()[self.traditional.0..self.traditional.1]
    }

    pub fn simplified(&self) -> &str {
        &self.line.as_ref()[self.simplified.0..self.simplified.1]
    }

    pub fn pinyin(&self) -> &str {
        &self.line.as_ref()[self.pinyin.0..self.pinyin.1]
    }

    pub fn definitions<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        let line = self.line.as_ref();
        let line = &line[self.definitions.0..self.definitions.1];
        let line = line.trim_matches('/');
        line.split('/')
    }
}

pub fn parse_dict_entry<T: AsRef<str>>(line: T) -> Option<DictEntry<T>> {
    let mut chars = line.as_ref().char_indices().peekable();

    // Skip comments and empty lines
    match chars.peek() {
        Some((_, '#')) => return None,
        None => return None,
        _ => (),
    }

    let traditional_start = chars.peek()?.0;
    loop {
        match chars.peek() {
            Some((_, ' ')) => break,
            None => return None,
            _ => {
                chars.next();
            }
        }
    }
    let traditional_end = chars.peek()?.0;

    // We know the next character is a space, so we can skip it
    match chars.next() {
        Some((_, ' ')) => (),
        _ => return None,
    };

    let simplified_start = chars.next()?.0;
    loop {
        match chars.peek() {
            Some((_, ' ')) => break,
            None => return None,
            _ => {
                chars.next();
            }
        }
    }
    let simplified_end = chars.peek()?.0;

    // We know the next character is a space, so we can skip it
    match chars.next() {
        Some((_, ' ')) => (),
        _ => return None,
    };

    // Expecting a '['
    match chars.next() {
        Some((_, '[')) => (),
        _ => return None,
    };

    let pinyin_start = chars.next()?.0;
    loop {
        match chars.peek() {
            Some((_, ']')) => break,
            None => return None,
            _ => {
                chars.next();
            }
        }
    }
    let pinyin_end = chars.peek()?.0;

    // We know the next character is a ']', so we can skip it
    match chars.next() {
        Some((_, ']')) => (),
        _ => return None,
    };

    // We know the next character is a space, so we can skip it
    match chars.next() {
        Some((_, ' ')) => (),
        _ => return None,
    };

    // We know the next character is a '/', so we can skip it
    match chars.next() {
        Some((_, '/')) => (),
        _ => return None,
    };

    let definitions_start = chars.next()?.0;

    let len = line.as_ref().len();

    Some(DictEntry {
        line,
        traditional: (traditional_start, traditional_end),
        simplified: (simplified_start, simplified_end),
        pinyin: (pinyin_start, pinyin_end),
        definitions: (definitions_start, len),
    })
}

/// Check if a line is a comment. Comments start with a '#'.
pub fn is_comment(line: &str) -> bool {
    let bytes = line.as_bytes();
    !bytes.is_empty() && bytes[0] == b'#'
}

/// Check if a line contains metadata. Metadata lines start with '#!'.
pub fn is_metadata(line: &str) -> bool {
    let bytes = line.as_bytes();
    bytes.len() > 1 && bytes[0] == b'#' && bytes[1] == b'!'
}

#[derive(Debug)]
pub enum Line {
    Comment(String),
    Metadata(String, String),
    Entry(DictEntry<String>),
    Empty,
    Incorrect,
}

pub fn parse_line<T: AsRef<str>>(line: T) -> Line {
    let line = line.as_ref();

    if line.is_empty() {
        Line::Empty
    } else if is_metadata(line) {
        // Strip the '#!' prefix
        let line = &line[2..].trim();

        // Split the line into key and value.
        let mut parts = line.splitn(2, '=');
        Line::Metadata(
            parts.next().unwrap().trim().to_string(),
            parts.next().unwrap().trim().to_string(),
        )
    } else if is_comment(line) {
        // Strip the '#' prefix
        Line::Comment(line[1..].trim().into())
    } else {
        match parse_dict_entry(line.into()) {
            Some(entry) => Line::Entry(entry),
            None => Line::Incorrect,
        }
    }
}

use std::io::BufRead;

pub fn parse_reader<T: std::io::Read>(f: T) -> impl Iterator<Item = DictEntry<String>> {
    let lines = std::io::BufReader::new(f).lines();
    let lines = lines.filter_map(|l| l.ok());
    let lines = lines.filter(|l| !is_comment(l));

    lines.filter_map(|x| parse_dict_entry(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dict_entry() {
        let line = "睡覺 睡觉 [shui4 jiao4] /to go to bed/to sleep/";
        let entry = parse_dict_entry(line).unwrap();
        assert_eq!(entry.traditional(), "睡覺");
        assert_eq!(entry.simplified(), "睡觉");
        assert_eq!(entry.pinyin(), "shui4 jiao4");
        assert_eq!(entry.definitions().nth(0), Some("to go to bed"));
        assert_eq!(entry.definitions().nth(1), Some("to sleep"));
    }
}
