use iced::widget::Text;
use iced::{Element, Task};
use iced_layershell::to_layer_message;

pub fn namespace() -> String {
    String::from("Charmander")
}

#[to_layer_message]
#[derive(Debug)]
pub enum Message {
    DoSmth,
    DoSmth2,
}

#[derive(Default)]
pub struct Charmander {
    //
}

pub fn update(_charmander: &mut Charmander, message: Message) -> Task<Message> {
    match message {
        Message::DoSmth => Task::none(),
        Message::DoSmth2 => Task::none(),
        _ => unreachable!(),
    }
}

pub fn view<'a>(_charmander: &'a Charmander) -> Element<'a, Message> {
    Element::new(Text::from("some text"))
}
