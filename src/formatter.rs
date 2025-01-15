use html5ever::parse_fragment;
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::{LocalName, QualName};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

fn print_tree_indented(indent: &str, handle: &Handle, print_html: bool) {
    let mut is_html = false;

    // Print the current node with indentation
    match handle.data {
        NodeData::Document => {
            // Skip printing document node
        }
        NodeData::Element { ref name, ref attrs, .. } => {
            is_html = name.local.to_string() == "html";
            // Skip printing html element
            if !is_html || print_html {
              print!("{}<{}", indent, name.local);
              
              // Print attributes if any exist
              for attr in attrs.borrow().iter() {
                  print!(" {}=\"{}\"", attr.name.local, attr.value);
              }
              println!(">");
            }
        }
        NodeData::Text { ref contents } => {
            let text = contents.borrow();
            if !text.trim().is_empty() {
                println!("{}{}", indent, text);
            }
        }
        NodeData::Comment { ref contents } => {
            println!("{}<!-- {} -->", indent, contents);
        }
        _ => {}
    }
    
    // Recursively print child nodes with increased indentation
    for child in handle.children.borrow().iter() {
      // Don't add prefix or newline to html element
      print_tree_indented(&format!("{}  ", indent), child, print_html);
    }
    
    // Print closing tag for elements
    if let NodeData::Element { ref name, .. } = handle.data {
        if !is_html || print_html {
            println!("{}</{}>", indent, name.local);
        }
    }
}

pub fn parse_and_print_html_indented(html: &str, wrap_with_html: bool) {
    // Create parser options
    let mut opts = ParseOpts::default();
    opts.tree_builder.drop_doctype = true;
    
    // Create an empty document
    let dom = parse_fragment(
        RcDom::default(),
        opts,
        QualName::new(None, Default::default(), LocalName::from("div")),
        vec![],
    )
      .from_utf8()
      .one(html.as_bytes());
    
    print_tree_indented("", &dom.document, wrap_with_html);
}

fn print_tree_unindented(handle: &Handle, print_html: bool) {
  let mut is_html = false;

  // Print the current node with indentation
  match handle.data {
      NodeData::Document => {
          // Skip printing document node
      }
      NodeData::Element { ref name, ref attrs, .. } => {
          is_html = name.local.to_string() == "html";
          // Skip printing html element
          if !is_html || print_html {
            print!("<{}", name.local);
            
            // Print attributes if any exist
            for attr in attrs.borrow().iter() {
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            print!(">");
          }
      }
      NodeData::Text { ref contents } => {
          let text = contents.borrow();
          if !text.trim().is_empty() {
              print!("{}", text.replace("\n", " ").trim());
          }
      }
      NodeData::Comment { ref contents } => {
          print!("<!-- {} -->", contents);
      }
      _ => {}
  }
  
  // Recursively print child nodes with increased indentation
  for child in handle.children.borrow().iter() {
    // Don't add prefix or newline to html element
    print_tree_unindented(child, print_html);
  }
  
  // Print closing tag for elements
  if let NodeData::Element { ref name, .. } = handle.data {
      if !is_html || print_html {
          print!("</{}>", name.local);
      }
  }
}

pub fn parse_and_print_html_unindented(html: &str, wrap_with_html: bool) {
  // Create parser options
  let mut opts = ParseOpts::default();
  opts.tree_builder.drop_doctype = true;
  
  // Create an empty document
  let dom = parse_fragment(
      RcDom::default(),
      opts,
      QualName::new(None, Default::default(), LocalName::from("div")),
      vec![],
  )
    .from_utf8()
    .one(html.as_bytes());
  
  print_tree_unindented(&dom.document, wrap_with_html);
  println!("");
}
