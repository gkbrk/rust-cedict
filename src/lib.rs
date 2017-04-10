#![feature(conservative_impl_trait)]
use std::io::{BufReader, BufRead, Read};

#[derive(Debug)]
pub struct DictEntry {
    pub traditional: String,
    pub simplified: String,
    pub pinyin: String,
    pub definitions: Vec<String>
}

pub fn parse_line<S: Into<String>>(line: S) -> Result<DictEntry, ()> {
    let line = line.into();
    let line = line.trim();

    let (traditional, line) = {
        let mut parts = line.splitn(2, " ");
        ( parts.next().ok_or(())?, parts.next().ok_or(())? )
    };

    let (simplified, line) = {
        let mut parts = line.splitn(2, " ");
        ( parts.next().ok_or(())?, parts.next().ok_or(())? )
    };

    let (pinyin, line) = {
        let pinyin_begin = line.find('[').ok_or(())? + 1;
        let pinyin_end = line.find(']').ok_or(())?;
        ( &line[pinyin_begin..pinyin_end], &line[pinyin_end+1..] )
    };

    let definitions = {
        let mut defs = Vec::new();
        let mut line = line.trim();
        while !line.is_empty() {
            let def_end = line.find('/').ok_or(())?;
            if !line[..def_end].is_empty() {
                defs.push(line[..def_end].to_string());
            }
            line = &line[def_end + 1..]
        }
        defs
    };
    
    Ok(DictEntry {
        traditional: traditional.to_string(),
        simplified: simplified.to_string(),
        pinyin: pinyin.to_string(),
        definitions: definitions
    })
}

pub fn parse_reader<T: Read>(f: T) -> impl Iterator<Item=DictEntry> {
    let bufread = BufReader::new(f);
    bufread.lines().filter_map(|x| x.ok())
        .map(|x| parse_line(x))
        .filter_map(|x| x.ok())
}

#[test]
fn test_parse_pinyin() {
    let line = "你好 你好 [ni3 hao3] /Hello!/Hi!/How are you?/";
    let parsed = parse_line(line).unwrap();

    assert_eq!(parsed.pinyin, "ni3 hao3");
}

#[test]
fn test_parse_simplified() {
    let line = "你好 你好 [ni3 hao3] /Hello!/Hi!/How are you?/";
    let parsed = parse_line(line).unwrap();

    assert_eq!(parsed.simplified, "你好");
}

#[test]
fn test_parse_traditional() {
    let line = "愛 爱 [ai4] /to love/to be fond of/to like/";
    let parsed = parse_line(line).unwrap();

    assert_eq!(parsed.traditional, "愛");
    assert_eq!(parsed.simplified, "爱");
}

#[test]
fn test_parse_reader() {
    let file = "你好 你好 [ni3 hao3] /Hello!/Hi!/How are you?/
                愛 爱 [ai4] /to love/to be fond of/to like/";

    for (i, word) in parse_reader(file.as_bytes()).enumerate() {
        match i {
            0 => {
                assert_eq!(word.simplified, "你好");
                assert_eq!(word.traditional, "你好");
                assert_eq!(word.pinyin, "ni3 hao3");
                assert_eq!(word.definitions[0], "Hello!");
            },
            1 => {
                assert_eq!(word.simplified, "爱");
                assert_eq!(word.traditional, "愛");
                assert_eq!(word.pinyin, "ai4");
                assert_eq!(word.definitions[1], "to be fond of");
            },
            _ => {}
        }
    }
}
