//use the traits feature to define the expected behavior of the events interface.
//the trait will contain the event hooks that the event handlers are supposed to implement.
//
//The only requirement for an object to be registered for an event is an implementation of the
//trait.
//
//This code structure is lifted from http://mattgathu.github.io/simple-events-hook-rust/

#[allow(unused_variables)]
trait Events {
    fn on_keyword(&self, s:&str) {}
    fn on_message(&self, s:&str) {}
    fn on_post_message(&self, s:&str) {}
    fn get_keyword(&self) -> &str { "" }
}

struct Parrot {
    keyword: String,
}

impl Parrot {
    fn new(key: String) -> Self
    {
        Self {
            keyword: key,
        }
    }
}

impl Events for Parrot {
    fn on_message(&self, s:&str)
    {
        println!("Parrot::on_message:   {}", s);
    }
    fn on_post_message(&self, s:&str)
    {
        println!("Parrot::post_message: {}", s);
    }
    fn on_keyword(&self, s:&str)
    {
        println!("Parrot::on_keyword:   {}", s);
    }
    fn get_keyword(&self) -> &str
    {
        &self.keyword
    }
}
struct Echo;

impl Events for Echo {
    fn on_message(&self, s:&str)
    {
        println!("Echo::on_message:     {:?}", s.split_at(1));
    }
    fn on_keyword(&self, s:&str)
    {
        println!("Echo::on_keyword:     {:?}", s.split_at(1));
    }
}

struct MessageHandler {
    hooks: Vec<Box<Events>>,
}

impl MessageHandler {
    fn new() -> Self
    {
        Self {
            hooks: Vec::new(),
        }
    }
    // putting a lifetime here is very important since we are putting the type that implements the
    // Events trait into a Box<T>
    // TODO: figure out what the <...> means
    // TODO: figure out if using constrained types would work here
    fn add_events_hook<E: Events + 'static>(&mut self, hook: E)
    {
        self.hooks.push(Box::new(hook));
    }
    fn handle(&self, s: &str)
    {
        for hook in &self.hooks {
            let key = hook.get_keyword();
            if !key.is_empty() && s.starts_with(key) {
                hook.on_keyword(s);
            }
        }
        for hook in &self.hooks {
            hook.on_message(s);
        }
        // do some mystery stuff
        for hook in &self.hooks {
            hook.on_post_message(s);
        }
    }
}

fn main()
{
    let what = String::from("this is some text");
    let mut msg_handler = MessageHandler::new();
    msg_handler.add_events_hook(Parrot::new(String::from("this")));
    msg_handler.add_events_hook(Echo);
    msg_handler.handle(&what);
}
