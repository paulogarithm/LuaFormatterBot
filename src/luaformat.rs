use lazy_static::lazy_static;
use regex::Error;
use regex::Regex;

lazy_static! {
    static ref LUA_PATTERNS: Vec<Result<Regex, Error>> = vec![
        Regex::new(r"^(?:(\`)?\s*(?:local\s+)?(\w+[:.])*(\w+\s*)([\+\-\*\/]?=\s*.+))$"),
        Regex::new(r"^(?:(\`)?\s*(local\s+)(\w+\s*)(,\s*\w+)*\s*([\+\-\*\/]?=\s*.+)?)$"),
        Regex::new(r"^(?:(\`)?\s*(\w+\s*)(,\s*\w+)*\s*([\+\-\*\/]?=\s*.+))$"),
        Regex::new(r"^\s*end$"),
        Regex::new(r"^(?:(\`)?\s*(?:local\s+)?function\s+(\w+[:.])*\w+\(.*?\))$"),
        Regex::new(r"^\s*(?:else)?if\s+.+?\s+then$"),
        Regex::new(r"^\s*else$"),
        Regex::new(r"^\s*(\w+[:.])*\w+\s*\(.*?\)\s*$"),
        Regex::new(r"^\s*return\s*.*?$"),
        Regex::new(r"^\s*for\s*(.+)do$"),
        Regex::new(r"^\s*while\s*(.+)do$"),
        Regex::new(r"^\s*repeat\s*(.*)$"),
        Regex::new(r"^\s*until\s*(.+)$"),
        Regex::new(r"^\s*--.*?$"),
        Regex::new(r"^\s*\w+\(.*?\)$"),
    ];
    static ref UNSURE_PATTERNS: Vec<Result<Regex, Error>> = vec![Regex::new(r"^[ \t]*$"),];
}

fn has_lua(line: &str) -> bool {
    let mut ok: bool = false;
    for pattern in LUA_PATTERNS.iter() {
        ok |= pattern.as_ref().unwrap().is_match(line);
        if ok {
            break;
        }
    }
    return ok;
}

fn has_unsure(line: &str) -> bool {
    let mut ok: bool = false;
    for pattern in UNSURE_PATTERNS.iter() {
        ok |= pattern.as_ref().unwrap().is_match(line);
        if ok {
            break;
        }
    }
    return ok;
}

pub fn get_blocks(lines: Vec<String>) -> Vec<(String, bool)> {
    let mut blocks: Vec<(String, bool)> = Vec::new();
    let (mut skip, mut previous, mut hasbeenonce) = (false, false, false);

    for line in lines {
        if skip {
            continue;
        }
        if line.contains("```lua") {
            skip = true;
            continue;
        }
        if line.contains("```") {
            skip = false;
            continue;
        }
        let ok = has_lua(line.as_str());
        let unsure = has_unsure(line.as_str());
        let mut status = false;
        println!(
            "  {} {}",
            if ok {
                "\x1b[32m✔\x1b[m"
            } else if unsure {
                "?"
            } else {
                "\x1b[31m✘\x1b[m"
            },
            line
        );
        status |= ok || (unsure && previous);
        hasbeenonce |= status;
        blocks.push((line, status));
        previous = status;
    }
    if !hasbeenonce {
        blocks = Vec::new()
    }
    return blocks;
}
