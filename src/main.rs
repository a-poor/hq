mod formatter;

use std::io::Read;
use std::io::IsTerminal;
use clap::Parser;
use scraper::{Html, Selector, ElementRef};
use formatter::{parse_and_print_html_indented, parse_and_print_html_unindented};


/// hq is a CLI to help you query HTML documents like `jq`.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The query to run
    query: String,

    /// Path to the HTML file to query (defaults to stdin)
    path: Option<String>,

    /// Select all matches (default)
    #[arg(short, long, default_value_t = true, group = "select")]
    all_matches: bool,

    /// Select the first match
    #[arg(short, long, default_value_t = false, group = "select")]
    first_match: bool,

    /// Return outer HTML (default)
    #[arg(short, long, default_value_t = true, group = "content")]
    outer_html: bool,

    /// Return inner HTML
    #[arg(short, long, default_value_t = false, group = "content")]
    inner_html: bool,

    /// Return text content
    #[arg(short, long, default_value_t = false, group = "content")]
    text: bool,

    /// Debug mode
    #[arg(short, long)]
    debug: bool,

    /// Indent output
    #[arg(long, default_value_t = false)]
    indent: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
enum SelectMode {
  #[default]
  All,
  One,
}

#[derive(Debug, Clone, Default, PartialEq)]
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
  Selector::parse(&query)
    .map_err(|e| format!("Invalid selector: {}", e))
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

fn print_result(result: Vec<String>, unindented: bool) {
  for r in result {
    if unindented {
      parse_and_print_html_unindented(&r, false);
    } else {
      parse_and_print_html_indented(&r, false);
    }
  }
}

fn main() {
  // Parse the CLI arguments
  let app = Args::parse();

  let content = match (app.outer_html, app.inner_html, app.text) {
    (true, false, false) => ContentMode::Outer,
    (false, true, false) => ContentMode::Inner,
    (false, false, true) => ContentMode::Text,
    _ => ContentMode::default(),
  };

  let select = match (app.all_matches, app.first_match) {
    (true, false) => SelectMode::All,
    (false, true) => SelectMode::One,
    _ => SelectMode::default(),
  };

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
  let result = match query_html(html, query, select, content) {
    Ok(result) => result,
    Err(e) => {
      eprintln!("Error querying HTML: {}", e);
      return;
    }
  };

  // Print the result
  print_result(result, !app.indent);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_query() {
    assert!(parse_query("div").is_ok());
    assert!(parse_query("").is_err());
    assert!(parse_query(".foo").is_ok());
  }
}
