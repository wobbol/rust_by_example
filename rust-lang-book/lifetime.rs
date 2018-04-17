struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}

//fn parse_context(context: Context) -> Result<(), &str> {
//    Parser { context: &context }.parse()
//}

fn main()
{
    let great = Context("This is a string to put in the context");
    let pear = Parser{context: &great};


    if let Err(s) = pear.parse() {
        println!("pear.parse: {}", s);
    }

//    let fork = Context("This is another string to put in a context");
//    if let Err(s) = parse_context(fork) {
//        println!("parse_context(): {}", s);
//    }
}

