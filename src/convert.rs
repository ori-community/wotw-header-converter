use std::iter;
use std::ops::Range;

pub fn convert(string: &str) -> String {
    let trailing_newlines = string.rfind(|c| c != '\n').map_or_else(|| string.len(), |index| string.len() - index - 1);
    let mut out = string.lines().map(|line| {
        let mut line = line.to_owned();
        convert_flags(&mut line);
        convert_messages(&mut line);
        line
    }).collect::<Vec<_>>().join("\n");
    out.extend(iter::repeat('\n').take(trailing_newlines));
    out
}

fn stringify_range(string: &mut String, range: Range<usize>) {
    string.replace_range(range.clone(), &format!("\"{}\"", &string[range]));
}

fn find_line_end(string: &str) -> usize {
    let line_break_or_comment = string.find("//").unwrap_or_else(|| string.find('\n').unwrap_or_else(|| string.len()));
    string[..line_break_or_comment].rfind(|c: char| !c.is_whitespace()).map_or(0, |index| index + 1)
}

enum State {
    Item,
    Item1,
    Command,
    Command1,
    Command2,
    WheelCommand,
}
fn find_last_item(string: &str) -> usize {
    let mut skip = 2;  // skip trigger
    let mut state = State::Item;
    let mut last_item_index = 0;
    for (index, char) in string.char_indices() {
        if skip > 0 {
            if char == '|' {
                skip -= 1;
                state = State::Item;
            }
            continue
        }
        if char == '|' { continue }

        match state {
            State::Item => {
                last_item_index = index;
                match char {
                    '1' => state = State::Item1,
                    '4' => state = State::Command,
                    _ => break,
                }
            },
            State::Item1 => {
                match char {
                    '6' => state = State::WheelCommand,
                    _ => break,
                }
            },
            State::Command => match char {
                '1' => state = State::Command1,
                '2' => state = State::Command2,
                _ => break,
            },
            State::Command1 => match char {
                '7' | '8' | '9' => skip = 4,  // skip ifs
                _ => break,
            },
            State::Command2 => match char {
                '4' => skip = 5,  // skip ifbox
                '5' | '6' | '7' => skip = 2,  // skip ifselfs
                _ => break,
            },
            State::WheelCommand => match char {
                '4' => skip = 4,  // skip wheel set item
                _ => break,
            },
        }
    }

    last_item_index
}

const FLAGS: &str = "Flags:";
fn convert_flags(string: &mut String) {
    if string.starts_with(FLAGS) {
        let mut start = FLAGS.len();
        loop {
            start += string[start..].find(|c: char| !c.is_whitespace()).unwrap_or(0);
            let (end, stop) = match string[start..].find(&[',', '\n']) {
                Some(index) => (start + index, !string[start + index..].starts_with(',')),
                None => (string.len(), true),
            };
            stringify_range(string, start..end);
            if stop { break }
            start = end + 3;
        }
    }
}

const MESSAGE: &str = "6|";
fn convert_messages(string: &mut String) {
    let index = find_last_item(string);
    if string[index..].starts_with(MESSAGE) {
        let start = index + MESSAGE.len();
        let end = find_line_end(string);

        let (flag_parts, message_parts): (Vec<_>, Vec<_>) = string[start..end].split('|')
            .partition(|&part| part.starts_with("f=")
            || part.starts_with("p=")
            || part == "mute"
            || part == "instant"
            || part == "quiet"
            || part == "noclear");

        let mut message = format!("\"{}\"", message_parts.join("|"));
        if !flag_parts.is_empty() {
            let flags = flag_parts.join("|");
            message = format!("{message}|{flags}");
        }
        string.replace_range(start..end, &message);
    }
}
