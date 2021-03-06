//I could not have written this without this example.
//The example code I used as a guide here:
//https://github.com/servo/html5ever/blob/master/html5ever/examples/print-rcdom.rs
//has a really clean style. I hope I can incorporate some of these
//structures into my future code.

#[macro_use] extern crate html5ever;
//extern crate html5ever;

use html5ever::{ParseOpts, parse_document};
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::TreeBuilderOpts;

use html5ever::rcdom::{RcDom, Handle, NodeData};
use html5ever::tendril::TendrilSink;
use std::ops::Deref;
use std::io;

fn get_attr(data :&NodeData, attribute: &str) -> Result<String, ()>
{
    match data {
        &NodeData::Element { ref attrs, .. } => {
            for attr in attrs.borrow().iter() {
                if attr.name.local.eq(attribute) {
                    let mut ret: String = String::new();
                    ret.insert_str(0, attr.value.deref());
                    return Ok(ret);
                }
            }
        },
        _ => (),
    }
    Err(())
}

fn find_id_in_tree(node: Handle, id: &str, attribute: &str) -> Result<String,()>
{
    match node.data {
        NodeData::Element { ref attrs, .. } => {
            for attr in attrs.borrow().iter() {
                if attr.name.local.eq("id") && attr.value.deref() == id {
                    return get_attr(&node.data, attribute);
                }
            }
        }
        _ => (),
    }
    for n in node.children.borrow().iter() {
        if let Ok(found) =  find_id_in_tree(n.clone(), id, attribute) {
            return Ok(found);
        }
    }
    return Err(());
}

fn print_tree(indent: u8, node: Handle)
{
    let mut c = 0;
    loop {
        if c == indent {
            break;
        }
        print!(" ");
        c = c + 1;
    }
    match node.data {
        NodeData::Document
            => println!("Document"),
        NodeData::Doctype { ref name, ref public_id, ref system_id }
            => println!("Doctype name: {} public_id: {} system_id: {}", name, public_id, system_id),
        NodeData::Text { ref contents }
            => println!("Text cont: {}", contents.borrow()),
        NodeData::Comment { ref contents }
            => println!("Comment cont: {}", contents),
        NodeData::Element { ref name, ref attrs, .. } => {
            if name.ns != ns!(html) { return; }
            print!("Element {}", name.local);
            for attr in attrs.borrow().iter() {
                if attr.name.ns != ns!() { return; }
                print!(" {} = {} ", attr.name.local, attr.value);
            }
            println!("");

        }
        NodeData::ProcessingInstruction { .. }
            => println!("ProcessingInstruction"),
    }
    for n in node.children.borrow().iter() {
        print_tree(indent + 2, n.clone());
    }
}

fn main() {
    let opts = ParseOpts {
        tokenizer: TokenizerOpts {
            exact_errors: false,
            discard_bom: true,
            profile: false,
            initial_state: None,
            last_start_tag_name: None
        },
        tree_builder: TreeBuilderOpts {
            exact_errors: false,
            scripting_enabled: false,
            iframe_srcdoc: false, //hmm
            drop_doctype: false, //hmm
            ignore_missing_rules: false,
            quirks_mode: html5ever::tree_builder::QuirksMode::NoQuirks,
        }
    };

    let stdin = io::stdin();
    let dom: RcDom = parse_document(RcDom::default(), opts).from_utf8().read_from(&mut stdin.lock()).unwrap();
    //print_tree(0, dom.document.clone());
    let id = "pdf";
    let attribute = "src";
     match find_id_in_tree(dom.document, id, attribute) {
        Ok(s) => {
            print!("{}", s);
        },
        Err(_) => panic!(),
    };
}

