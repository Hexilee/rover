use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox, Clipboard, Column,
    Command, Container, Element, Font, HorizontalAlignment, Length, Row, Scrollable, Settings,
    Text, TextInput,
};
use serde::{Deserialize, Serialize};
use url::Url;

pub fn main() -> iced::Result {
    Tab::run(Settings::default())
}

#[derive(Debug, Default)]
struct Tab {
    is_loading: bool,
    address: String,
    input: text_input::State,
    address_stack: Vec<Url>,
    back_button: button::State,
    refresh_button: button::State,
    cancel_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    Empty,
    Load,
}

impl Application for Tab {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        format!("Rover Browser")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let input = TextInput::new(&mut self.input, "enter address", &self.address, |_| {
            Message::Empty
        })
        .padding(6)
        .size(18)
        .on_submit(Message::Load);

        let address_row = Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(Button::new(
                &mut self.back_button,
                Row::new().push(back_icon()),
            ))
            .push(Button::new(
                &mut self.refresh_button,
                Row::new().push(refresh_icon()),
            ))
            .push(Container::new(input).width(Length::Fill).max_width(800));

        Column::new()
            .padding(5)
            // .push(title)
            .push(address_row)
            .into()
        // .push(controls)
        // .push(tasks);
    }
}

fn loading_message<'a>() -> Element<'a, Message> {
    Container::new(
        Text::new("Loading...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}

fn empty_message<'a>(message: &str) -> Element<'a, Message> {
    Container::new(
        Text::new(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(HorizontalAlignment::Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Units(200))
    .center_y()
    .into()
}

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

fn edit_icon() -> Text {
    icon('\u{F303}')
}

fn delete_icon() -> Text {
    icon('\u{F1F8}')
}

fn back_icon() -> Text {
    icon('\u{2190}')
}

fn refresh_icon() -> Text {
    icon('\u{27F3}')
}

fn cancel_icon() -> Text {
    icon('\u{1F5D9}')
}

#[derive(Debug, Clone)]
enum LoadError {
    FileError,
    FormatError,
}

#[derive(Debug, Clone)]
enum SaveError {
    FileError,
    WriteError,
    FormatError,
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Filter { selected: bool },
        Icon,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Filter { selected } => {
                    if *selected {
                        button::Style {
                            background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.7))),
                            border_radius: 10.0,
                            text_color: Color::WHITE,
                            ..button::Style::default()
                        }
                    } else {
                        button::Style::default()
                    }
                }
                Button::Icon => button::Style {
                    text_color: Color::from_rgb(0.5, 0.5, 0.5),
                    ..button::Style::default()
                },
                Button::Destructive => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                    border_radius: 5.0,
                    text_color: Color::WHITE,
                    shadow_offset: Vector::new(1.0, 1.0),
                    ..button::Style::default()
                },
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                text_color: match self {
                    Button::Icon => Color::from_rgb(0.2, 0.2, 0.7),
                    Button::Filter { selected } if !selected => Color::from_rgb(0.2, 0.2, 0.7),
                    _ => active.text_color,
                },
                shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                ..active
            }
        }
    }
}
