use evdev::KeyCode;
use iced::widget::{Column, Text};
use iced::{Element, Event, Subscription, Task, event, window};
use iced_layershell::to_layer_message;

use crate::config;
use crate::state::{Candidate, State};

#[expect(
    clippy::must_use_candidate,
    reason = "callback passed to iced_layershell, never called directly"
)]
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
        self.overlay = Some((candidates, selection));
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

#[expect(
    clippy::must_use_candidate,
    reason = "callback passed to iced_layershell, never called directly"
)]
pub fn view(charmander: &Charmander) -> Element<'_, Message> {
    match &charmander.overlay {
        Some((candidates, selection)) => candidates
            .iter()
            .enumerate()
            .fold(Column::new(), |column, (i, candidate)| {
                let label = Text::new(candidate.name.clone());
                let label = if i == *selection {
                    label.color(config::SELECTED_COLOR)
                } else {
                    label
                };
                column.push(label)
            })
            .into(),
        None => Column::new().into(),
    }
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "signature is dictated by iced::event::listen_with"
)]
fn mouse_to_message(
    event: Event,
    _status: event::Status,
    _window_id: window::Id,
) -> Option<Message> {
    use iced::mouse::{
        Button::Left as LeftButton, Button::Right as RightButton,
        Event::ButtonPressed as MouseButtonPressed,
    };

    match event {
        Event::Mouse(MouseButtonPressed(LeftButton)) => {
            Some(Message::StateMachineChanged(State::Accent {
                target_key: KeyCode::KEY_L,
                symbols: config::test_symbols(),
                selection: 0,
            }))
        }
        Event::Mouse(MouseButtonPressed(RightButton)) => {
            Some(Message::StateMachineChanged(State::Idle))
        }
        _ => None,
    }
}

pub fn subscription(_charmander: &Charmander) -> Subscription<Message> {
    event::listen_with(mouse_to_message)
}
