use std::io::Read;
use std::io::IsTerminal;
use clap::{Parser, ValueEnum};
use scraper::{Html, Selector, ElementRef};

/// hq is a CLI to help you query HTML documents like `jq`.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The query to run
    query: String,

    /// Path to the HTML file to query (defaults to stdin)
    path: Option<String>,

    /// Output mode
    #[arg(short, long, value_enum, default_value_t = SelectMode::All)]
    select: SelectMode,

    /// Return mode
    #[arg(short, long, value_enum, default_value_t = ContentMode::Outer)]
    content: ContentMode,

    /// Debug mode
    #[arg(short, long)]
    debug: bool,
}

#[derive(Debug, Clone, Default, ValueEnum, PartialEq)]
enum SelectMode {
  #[default]
  All,
  One,
}

#[derive(Debug, Clone, Default, ValueEnum, PartialEq)]
enum ContentMode {
  #[default]
  Outer,
  Inner,
  Text,
}

fn has_stdin_data() -> bool {
  !std::io::stdin().is_terminal()
}

enum InputError {
  NoInput,
  NoFile,
  StdinError(std::io::Error),
  FileError(std::io::Error),
}

fn read_input(path: Option<String>) -> Result<String, InputError> {
    match (path, has_stdin_data()) {
      (Some(p), _) => {
        // User provided a file path...
        // Check if the file exists
        if !std::path::Path::new(&p).exists() {
          return Err(InputError::NoFile);
        }

        // Read the file
        let raw = std::fs::read_to_string(&p).map_err(InputError::FileError)?;
        Ok(raw)
      }
      (None, true) => {
          // No file specified but stdin is piped...
          let mut buffer = String::new();
          
          // Read the stdin
          std::io::stdin().read_to_string(&mut buffer).map_err(InputError::StdinError)?;
          Ok(buffer)
      }
      (None, false) => {
          // No file and no stdin - show error
          Err(InputError::NoInput)
      },
    }
}

fn parse_query(query: &str) -> Result<Selector, String> {
  Selector::parse(&query).map_err(|e| format!("Invalid selector: {}", e))
}

fn parse_element(element: ElementRef, content: &ContentMode) -> String {
  match content {
    ContentMode::Outer => element.html().to_string(),
    ContentMode::Inner => element.inner_html().to_string(),
    ContentMode::Text => element.text().map(|s| s.to_owned()).collect::<Vec<_>>().join(""),
  }
}

fn query_html(html: Html, query: Selector, select: SelectMode, content: ContentMode) -> Result<Vec<String>, String> {
  let mut res: Vec<String> = Vec::new();
  for m in html.select(&query) {
    res.push(parse_element(m, &content));
    if matches!(select, SelectMode::One) {
      break;
    }
  }
  Ok(res)
}

fn print_result(result: Vec<String>) {
  for r in result {
    println!("{}", deindent(&r));
  }
}

fn deindent(text: &str) -> String {
  let lines = text
    .lines()
    .collect::<Vec<_>>();
  let min_space_pfx = lines
    .iter()
    .map(|line| line.len() - line.trim_start().len())
    .min()
    .unwrap_or(0);
  lines
    .iter()
    .map(|line| line[min_space_pfx..].to_string())
    .collect::<Vec<_>>()
    .join("\n")
}

fn main() {
  // Parse the CLI arguments
  let app = Args::parse();

  // Read the input
  let raw = match read_input(app.path) {
    Ok(html) => html,
    Err(e) => {
      match e {
        InputError::NoInput => {
          eprintln!("No input provided");
        },
        InputError::NoFile => {
          eprintln!("No file provided");
        },
        InputError::StdinError(err) => {
          eprintln!("Error reading from stdin: {}", err);
        },
        InputError::FileError(err) => {
          eprintln!("Error reading from file: {}", err);
        }
      }
      return;
    }
  };

  let html = Html::parse_fragment(&raw);

  // Parse the query
  let query = match parse_query(&app.query) {
    Ok(query) => query,
    Err(e) => {
      eprintln!("Error parsing query: {}", e);
      return;
    }
  };

  // Query the HTML
  let result = match query_html(html, query, app.select, app.content) {
    Ok(result) => result,
    Err(e) => {
      eprintln!("Error querying HTML: {}", e);
      return;
    }
  };

  // Print the result
  print_result(result);
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn test_read_input() {}

  #[test]
  fn test_parse_query() {
    assert!(parse_query("div").is_ok());
    assert!(parse_query("").is_err());
    assert!(parse_query(".foo").is_ok());
  }

  // #[test]
  // fn test_parse_element() {}

  // #[test]
  // fn test_query_html() {}

  #[test]
  fn test_deindent() {
    assert_eq!(deindent("  foo\n  bar"), "foo\nbar");
    assert_eq!(deindent("foo\n  bar"), "foo\n  bar");
    assert_eq!(deindent("foo\nbar"), "foo\nbar");
    assert_eq!(deindent("foo\n  bar\n    baz"), "foo\n  bar\n    baz");
  }
}
