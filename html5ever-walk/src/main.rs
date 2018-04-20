#[macro_use] extern crate html5ever;

use html5ever::{ParseOpts, parse_document};
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::TreeBuilderOpts;

use html5ever::rcdom::{RcDom, Handle, NodeData};
use html5ever::tendril::TendrilSink;

fn walk_tree(indent: u8, node: Handle)
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
        NodeData::Element { ref name, ref attrs, ref template_contents, ref mathml_annotation_xml_integration_point } => {
            if name.ns != ns!(html) { return; }
            print!("Element {}", name.local);
            for attr in attrs.borrow().iter() {
                if attr.name.ns != ns!() { return; }
                print!(" {} = {} ", attr.name.local, attr.value);
            }
            println!("");

        }
        NodeData::ProcessingInstruction { ref target, ref contents }
            => println!("ProcessingInstruction"),
    }
    for n in node.children.borrow().iter() {
        walk_tree(indent + 2, n.clone());
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

    let res_content = String::from(" <!DOCTYPE html><!-- hmmmm --> <html> <body> <h1 id=\"goodboy\">ooh you made it</h1> </body> </html>");

    let dom: RcDom = parse_document(RcDom::default(), opts).from_utf8().read_from(&mut res_content.as_bytes()).unwrap();
    walk_tree(0, dom.document);
}
