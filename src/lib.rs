#[derive(Debug)]
pub struct DictEntry<'a> {
    traditional: &'a str,
    simplified: &'a str,
    pinyin: &'a str,
    definitions: Vec<&'a str>
}

pub fn parse_line(line: &str) -> Result<DictEntry, ()> {
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
                defs.push(&line[..def_end]);
            }
            line = &line[def_end + 1..]
        }
        defs
    };
    
    Ok(DictEntry {
        traditional: traditional,
        simplified: simplified,
        pinyin: pinyin,
        definitions: definitions
    })
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
