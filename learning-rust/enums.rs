enum FlashMessage {
    Success,
    Warning{ catagory: i32, message: String },
    Error(String)
}
use std::fmt;

impl fmt::Display for FlashMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FlashMessage::Success => write!(f, "Form Submitted correctly."),
            &FlashMessage::Warning {catagory, ref message} => write!(f,"Warning: {} - {}.", catagory, message),
            &FlashMessage::Error(ref msg) => write!(f, "Error: {}.", msg)
        }
    }
}

fn main()
{
    let mut form_status = FlashMessage::Success;
    println!("{}", form_status);
    form_status = FlashMessage::Warning {catagory: 2, message: String::from("A picture of your left foot is required")};
    println!("{}", form_status);
    form_status = FlashMessage::Error(String::from("Hotdog lost in transit"));
    println!("{}", form_status);

}
