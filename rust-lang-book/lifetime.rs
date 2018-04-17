struct Context<'s>(&'s str);

// 's: 'c - 's is guaranteed to outlive 'c
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

//the output of .parse will be a &str of context which was passed in.
//Without different lifetimes for Parser and Context the output of
// .parse becomes owned by the Parser that is local to this function
// making this kind of information exchange impossible.
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

fn main()
{
    let great = Context("This is a string to put in the context");
    let pear = Parser{context: &great};


    if let Err(s) = pear.parse() {
        println!("pear.parse: {}", s);
    }

    let fork = Context("This is another string to put in a context");
    if let Err(s) = parse_context(fork) {
        println!("parse_context(): {}", s);
    }
}

