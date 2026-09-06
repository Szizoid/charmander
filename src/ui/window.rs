use iced::widget::{Column, Text};
use iced::{Element, Task};
use iced_layershell::to_layer_message;

use crate::config;
use crate::state::machine::{Candidate, State};

pub fn namespace() -> String {
    String::from(config::APP_NAME)
}

#[to_layer_message]
#[derive(Debug)]
pub enum Message {
    StateMachineChanged(State),
}

#[derive(Default)]
pub struct Charmander {
    overlay: Option<(Vec<Candidate>, usize)>,
}

impl Charmander {
    pub fn accent(&mut self, candidates: Vec<Candidate>, selection: usize) {
        self.overlay = Some((candidates, selection))
    }
    pub fn hide(&mut self) {
        self.overlay = None;
    }
}

pub fn update(charmander: &mut Charmander, message: Message) -> Task<Message> {
    match message {
        Message::StateMachineChanged(State::Accent {
            target_key: _,
            symbols,
            selection,
        }) => {
            charmander.accent(symbols, selection);
            Task::none()
        }
        // Idle or Tracking: need to hide the window if it isn't hidden already
        // (it may still be showing from a previous Accent state).
        Message::StateMachineChanged(_) => {
            charmander.hide();
            Task::none()
        }
        Message::AnchorChange(_) => todo!(),
        Message::SetInputRegion(_) => todo!(),
        Message::AnchorSizeChange(_, _) => todo!(),
        Message::LayerChange(_) => todo!(),
        Message::MarginChange(_) => todo!(),
        Message::SizeChange(_) => todo!(),
        Message::ExclusiveZoneChange(_) => todo!(),
        Message::KeyboardInteractivityChange(_) => todo!(),
        Message::VirtualKeyboardPressed { key: _ } => todo!(),
    }
}

pub fn view<'a>(charmander: &'a Charmander) -> Element<'a, Message> {
    match &charmander.overlay {
        Some((candidates, selection)) => candidates
            .iter()
            .enumerate()
            .fold(Column::new(), |column, (i, candidate)| {
                let label = Text::new(candidate.name.clone());
                let label = if i == *selection {
                    label.color(iced::Color::from_rgb(
                        config::SELECTED_COLOR_RED,
                        config::SELECTED_COLOR_GREEN,
                        config::SELECTED_COLOR_BLUE,
                    ))
                } else {
                    label
                };
                column.push(label)
            })
            .into(),
        None => Column::new().into(),
    }
}
