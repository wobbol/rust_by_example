// paraphrased from here: https://stackoverflow.com/questions/37572734/how-can-i-implement-the-observer-pattern-in-rust
//
// in languages with a GC it is typical to have
// Observeable -> Observer to notify
// Observer -> Observeable to unregister
//
// Rust is not GC'd so we must do something different

use std::rc::Weak;
use std::rc::Rc;

struct Item {
    o: Weak<Observer>,
    cmd: String,
}

struct Action {
    item: Vec<Item>,
}

impl Action {
    fn new() -> Action
    {
        Action {
            item: Vec::new(),
        }
    }

//    fn search(&self, s: &str)
//    {
//        for i in self.item {
//            if s.starts_with(&i.cmd) {
//                i.o.notifiy(s, i.cmd.len());
//                return;
//            }
//        }
//    }
}

impl Observeable for Action {
    fn register(&mut self, observer: Weak<Observer>, cmd_string: String)
    {
        self.item.push(Item{o:observer, cmd:cmd_string});
    }
}
trait Observeable {
    fn register(&mut self, observer: Weak<Observer>, cmd_string: String);
}
trait Observer {
    fn notify(&self, s: &str, len: usize);
}
struct Echo;
impl Observer for Echo {
    fn notify(&self, s: &str, len: usize)
    {
        if s.len() > len {
            let (_, stuff) = s.split_at(len);
            println!("echo: {}", stuff);
        }
    }
}
#[derive(Debug)]
struct Parrot;
impl Observer for Parrot {
    fn notify(&self, s: &str, _len: usize)
    {
        println!("parrot: {}", s);
    }
}

fn main()
{
    let input = String::from("/parrot, twine, paint, and twiddle");
    let what = Parrot;
    what.notify(&input,5);

    //TODO: find a way to pass Parrot and Echo as Weak<Observer> to register
    let a: Action = Action::new();
    //let rc = &Rc::new(echo as Observer);
    //a.register(Rc::downgrade(rc),String::from("/echo"));
    //a.register(&parrot,String::from("/parrot"));
    //a.search(&input);
}
