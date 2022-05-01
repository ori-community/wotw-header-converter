pub fn convert(string: &str) -> String {
    string.lines().map(|line| {
        let mut line = line.to_owned();
        convert_flags(&mut line);
        line
    }).collect::<Vec<_>>().join("\n")
}

const FLAGS: &str = "Flags:";
fn convert_flags(string: &mut String) {
    if let Some(mut start) = string.find(FLAGS) {
        start += FLAGS.len();
        loop {
            start += string[start..].find(|c: char| !c.is_whitespace()).unwrap_or(0);
            let (end, stop) = match string[start..].find(&[',', '\n']) {
                Some(index) => (start + index, !string[start + index..].starts_with(',')),
                None => (string.len(), true),
            };
            string.replace_range(start..end, &format!("\"{}\"", &string[start..end]));
            if stop { break }
            start = end + 3;
        }
    }
}
