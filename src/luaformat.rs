use lazy_static::lazy_static;
use regex::Error;
use regex::Regex;

type Pattern = Result<Regex, Error>;

lazy_static! {
  /// Patterns often seen in Lua source code.
  static ref LUA_PATTERNS: Vec<Pattern> = vec![
    // Variable definitions
    Regex::new(r"^(?:(\`)?\s*(?:local\s+)?(\w+[:.])*(\w+\s*)([\+\-\*\/]?=\s*.+))$"),
    Regex::new(r"^(?:(\`)?\s*(local\s+)(\w+\s*)(,\s*\w+)*\s*([\+\-\*\/]?=\s*.+)?)$"),

    // Function definitions
    Regex::new(r"^(?:(\`)?\s*(\w+\s*)(,\s*\w+)*\s*([\+\-\*\/]?=\s*.+))$"),
    Regex::new(r"^\s*end$"),
    Regex::new(r"^(?:(\`)?\s*(?:local\s+)?function\s+(\w+[:.])*\w+\(.*?\))$"),

    // Control structures
    Regex::new(r"^\s*(?:else)?if\s+.+?\s+then$"),
    Regex::new(r"^\s*else$"),

    // Function calls
    Regex::new(r"^\s*(\w+[:.])*\w+\s*\(.*?\)\s*$"),
    Regex::new(r"^\s*\w+\(.*?\)$"),

    // Other keywords
    Regex::new(r"^\s*return\s*.*?$"),
    Regex::new(r"^\s*for\s*(.+)do$"),
    Regex::new(r"^\s*while\s*(.+)do$"),
    Regex::new(r"^\s*repeat\s*(.*)$"),
    Regex::new(r"^\s*until\s*(.+)$"),

    // Comments
    Regex::new(r"^\s*--.*?$"),
  ];

  /// Patterns that are <u>probably</u> Lua source code.
  static ref UNSURE_PATTERNS: Vec<Pattern> = vec![
    Regex::new(r"^[ \t]*$"),
  ];
}

/// Returns `true` if the given line matches any of the given patterns.
fn has_pattern(line: &str, patterns: Vec<Pattern>) -> bool {
  patterns.iter().any(|pattern| pattern.as_ref().unwrap().is_match(line))
}

/// Returns a status symbol based on 2 `bool`s passed to the function.
fn get_status_symbol(ok: bool, unsure: bool) -> &'static str {
  if ok {
    "\x1b[32m✔\x1b[m"
  } else if unsure {
    "\x1b[33m?\x1b[m"
  } else {
    "\x1b[31m✘\x1b[m"
  }
}

pub fn extract_codeblocks(lines: Vec<String>) -> Vec<(String, bool)> {
  let mut codeblocks: Vec<(String, bool)> = Vec::new();
  let mut in_lua_codeblock = false;
  let mut skip_codeblock = false;
  let mut previous_line_was_codeblock = false;

  for line in lines {
    if skip_codeblock {
      continue;
    }

    if line.contains("```lua") {
      skip_codeblock = true;
      continue;
    }

    if line.contains("```") {
      skip_codeblock = false;
      continue;
    }

    let ok = has_pattern(line.as_str(), LUA_PATTERNS.to_vec());
    let unsure = has_pattern(line.as_str(), UNSURE_PATTERNS.to_vec());
    let mut status = false;

    println!("  {} {}", get_status_symbol(ok, unsure), line);

    status |= ok || (unsure && previous_line_was_codeblock);
    in_lua_codeblock |= status;
    codeblocks.push((line, status));
    previous_line_was_codeblock = status;
  }

  if !in_lua_codeblock {
    codeblocks = Vec::new()
  }

  codeblocks
}
