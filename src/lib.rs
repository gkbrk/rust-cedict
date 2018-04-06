//! cedict is a Rust crate for reading and writing the CC-CEDICT
//! Chinese-English dictionary format. It can be used to implement Chinese
//! dictionaries in Rust. It can also serve as a tool for automating
//! maintenance to the CEDICT project.
//!
//! # Examples
//! ```
//! let line = "你好 你好 [ni3 hao3] /Hello!/Hi!/How are you?/";
//! let parsed = cedict::parse_line(line).unwrap();
//!
//! assert_eq!(parsed.definitions().next(), Some("Hello!"));
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

use std::io::{BufRead, BufReader, Read};

/// A struct that contains all fields of a CEDICT definition
#[derive(Debug)]
pub struct DictEntry {
    line: String,
    traditional: Slice,
    simplified: Slice,
    pinyin: Slice,
    definitions: Vec<Slice>,
}

type Slice = (usize, usize);

impl DictEntry {
    /// Gets the traditional Chinese characters for the entry.
    ///
    /// ```
    /// let line = "學習 学习 [xue2 xi2] /to learn/to study/";
    /// let dict = cedict::parse_line(line).unwrap();
    ///
    /// assert_eq!(dict.traditional(), "學習");
    /// ```
    pub fn traditional(&self) -> &str {
        self.slice(&self.traditional)
    }

    /// Gets the simplified traditional characters for the entry.
    ///
    /// ```
    /// let line = "學習 学习 [xue2 xi2] /to learn/to study/";
    /// let dict = cedict::parse_line(line).unwrap();
    ///
    /// assert_eq!(dict.simplified(), "学习");
    /// ```
    pub fn simplified(&self) -> &str {
        self.slice(&self.simplified)
    }

    /// Gets the pinyin form of the entry.
    ///
    /// ```
    /// let line = "學習 学习 [xue2 xi2] /to learn/to study/";
    /// let dict = cedict::parse_line(line).unwrap();
    ///
    /// assert_eq!(dict.pinyin(), "xue2 xi2");
    /// ```
    pub fn pinyin(&self) -> &str {
        self.slice(&self.pinyin)
    }

    /// Returns an iterator over the definitions of the entry.
    ///
    /// ```
    /// let line = "學習 学习 [xue2 xi2] /to learn/to study/";
    /// let dict = cedict::parse_line(line).unwrap();
    ///
    /// assert_eq!(dict.definitions().nth(0), Some("to learn"));
    /// assert_eq!(dict.definitions().nth(1), Some("to study"));
    /// ```
    pub fn definitions(&self) -> impl Iterator<Item = &str> {
        self.definitions.iter().map(move |x| self.slice(x))
    }

    fn slice(&self, slice: &Slice) -> &str {
        &self.line[slice.0..slice.1]
    }

    /// Creates a new `DictEntry`. This can be used to add new entries to the
    /// file.
    pub fn new(trad: &str, simp: &str, pinyin: &str, defs: Vec<&str>) -> DictEntry {
        let mut line = String::new();

        line.push_str(trad);
        line.push(' ');
        line.push_str(simp);
        line.push_str(" [");
        line.push_str(pinyin);
        line.push_str("] /");

        for def in defs {
            line.push_str(def);
            line.push('/');
        }

        parse_line(line).unwrap()
    }

    /// Formats a DictEntry into a CEDICT formatted line. This function can be
    /// used to modify or create CEDICT files.
    pub fn to_string(&self) -> &str {
        &self.line
    }
}

/// Parses a line in the CEDICT format into a `DictEntry`
///
/// # Examples
/// ```
/// let line = "你好 你好 [ni3 hao3] /Hello!/Hi!/How are you?/";
/// let parsed = cedict::parse_line(line).unwrap();
///
/// assert_eq!(parsed.definitions().nth(0), Some("Hello!"));
/// assert_eq!(parsed.definitions().nth(1), Some("Hi!"));
/// ```
pub fn parse_line<S: Into<String>>(line: S) -> Result<DictEntry, ()> {
    let line = line.into();
    let line = line.trim();
    let line_orig = line;
    let lineptr = line.as_ptr();
    let linelen = line.len();

    // Handle file comments
    // They are currently ignored
    if line.starts_with('#') {
        return Err(());
    }

    let (traditional, line) = {
        let mut parts = line.splitn(2, ' ');
        (parts.next().ok_or(())?, parts.next().ok_or(())?)
    };

    let (simplified, line) = {
        let mut parts = line.splitn(2, ' ');
        (parts.next().ok_or(())?, parts.next().ok_or(())?)
    };

    let (pinyin, line) = {
        let pinyin_begin = line.find('[').ok_or(())? + 1;
        let pinyin_end = line.find(']').ok_or(())?;
        (&line[pinyin_begin..pinyin_end], &line[pinyin_end + 1..])
    };

    let toslice = |a: &str| {
        let start = a.as_ptr() as usize - lineptr as usize;
        assert!(start < linelen);
        (start, start + a.len())
    };

    let definitions = {
        let mut defs = Vec::new();
        let mut line = line.trim();
        while !line.is_empty() {
            let def_end = line.find('/').ok_or(())?;
            if !line[..def_end].is_empty() {
                defs.push(toslice(&line[..def_end]));
            }
            line = &line[def_end + 1..]
        }
        defs
    };

    Ok(DictEntry {
        line: line_orig.to_string(),
        traditional: toslice(traditional),
        simplified: toslice(simplified),
        pinyin: toslice(pinyin),
        definitions,
    })
}

/// Returns an iterator over Readers, which can be open files, byte arrays
/// or anything else that implements Read
///
/// # Examples
/// ```
/// use std::fs::File;
///
/// let f = match File::open("cedict.txt") {
///     Ok(f) => f,
///     Err(_) => { return; }
/// };
///
/// for dict_entry in cedict::parse_reader(f) {
///     println!("Read the definition of {}. It means {}.", dict_entry.simplified(),
///       dict_entry.definitions().next().unwrap());
/// }
/// ```
pub fn parse_reader<T: Read>(f: T) -> impl Iterator<Item = DictEntry> {
    let bufread = BufReader::new(f);
    bufread
        .lines()
        .filter_map(|x| x.ok())
        .map(parse_line)
        .filter_map(|x| x.ok())
}

#[test]
fn test_parse_pinyin() {
    let line = "你好 你好 [ni3 hao3] /Hello!/Hi!/How are you?/";
    let parsed = parse_line(line).unwrap();

    assert_eq!(parsed.pinyin(), "ni3 hao3");
}

#[test]
fn test_parse_simplified() {
    let line = "你好 你好 [ni3 hao3] /Hello!/Hi!/How are you?/";
    let parsed = parse_line(line).unwrap();

    assert_eq!(parsed.simplified(), "你好");
}

#[test]
fn test_parse_traditional() {
    let line = "愛 爱 [ai4] /to love/to be fond of/to like/";
    let parsed = parse_line(line).unwrap();

    assert_eq!(parsed.traditional(), "愛");
    assert_eq!(parsed.simplified(), "爱");
}

#[test]
fn test_parse_reader() {
    let file = "你好 你好 [ni3 hao3] /Hello!/Hi!/How are you?/
                愛 爱 [ai4] /to love/to be fond of/to like/";

    for (i, word) in parse_reader(file.as_bytes()).enumerate() {
        match i {
            0 => {
                assert_eq!(word.simplified(), "你好");
                assert_eq!(word.traditional(), "你好");
                assert_eq!(word.pinyin(), "ni3 hao3");
                assert_eq!(word.definitions().next(), Some("Hello!"));
            }
            1 => {
                assert_eq!(word.simplified(), "爱");
                assert_eq!(word.traditional(), "愛");
                assert_eq!(word.pinyin(), "ai4");
                assert_eq!(word.definitions().nth(1), Some("to be fond of"));
            }
            _ => {}
        }
    }
}

#[test]
fn test_to_string() {
    let definition = DictEntry::new("愛", "爱", "ai4", vec!["to love", "to like"]);

    assert_eq!(definition.to_string(), "愛 爱 [ai4] /to love/to like/");
}
