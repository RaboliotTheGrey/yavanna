// /*
//  * Resources:
//  * https://github.com/hecrj/iced/blob/master/examples/todos/src/main.rs
//  * https://github.com/hecrj/iced/blob/master/examples/progress_bar/src/main.rs
//  * https://github.com/hecrj/iced/tree/master/examples
//  */
//
use crate::core::sleep;
//
// use iced::{Sandbox, Element, Column, Align, Button, Text, TextInput};

use iced::{Sandbox, Element, button, Column, Button, Text, Settings, Align, Row, Checkbox, Container, Length, window};

pub fn run() {
    let settings = Settings {
        window: window::Settings {
            size: (500, 300),
            resizable: false,
            decorations: true,
        },
        ..Default::default()
    };
    Yavanna::run(settings)
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    QuicktimeAddThirtyMinPressed,
    QuicktimeAddOneHourPressed,
    QuicktimeAddTwoHoursPressed,

    IncrementHourPressed,
    DecrementHourPressed,
    IncrementMinutePressed,
    DecrementMinutePressed,

    ActivateTimer(bool),

    Sleep,
    Cancel,
}

pub struct Yavanna {
    pub hours: u32,
    pub minutes: u32,
    pub timer: bool,
    pub active: bool,

    timer_tick: button::State,

    increment_hour_button: button::State,
    decrement_hour_button: button::State,
    increment_minute_button: button::State,
    decrement_minute_button: button::State,

    quicktime_thirty_min_button: button::State,
    quicktime_one_hour_button: button::State,
    quicktime_two_hours_button: button::State,

    sleep_button: button::State,
    cancel_button: button::State,
}

impl Sandbox for Yavanna {
    type Message = Message;

    fn new() -> Self {
        Yavanna {
            hours: 0,
            minutes: 0,
            timer: true,
            active: false,
            timer_tick: Default::default(),
            increment_hour_button: Default::default(),
            decrement_hour_button: Default::default(),
            increment_minute_button: Default::default(),
            decrement_minute_button: Default::default(),
            quicktime_thirty_min_button: Default::default(),
            quicktime_one_hour_button: Default::default(),
            quicktime_two_hours_button: Default::default(),
            sleep_button: Default::default(),
            cancel_button: Default::default(),
        }
    }

    fn title(&self) -> String {
        format!("Yavanna - v{}", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::QuicktimeAddThirtyMinPressed => {
                let minutes = self.minutes + 30;
                if minutes == 60 {
                    self.hours += 1;
                    self.minutes = 0;
                } else if minutes > 60 {
                    // not the safest way but for now, we can directly increment +1h30
                    self.hours += minutes / 60;
                    self.minutes += minutes % 60;
                } else {
                    self.minutes = minutes;
                }
            }
            Message::QuicktimeAddOneHourPressed => {
                self.hours += 1;
            }
            Message::QuicktimeAddTwoHoursPressed => {
                self.hours += 2;
            }
            Message::IncrementHourPressed => {
                self.hours += 1;
            }
            Message::DecrementHourPressed => {
                if self.hours > 0 {
                    self.hours -= 1;
                }
            }
            Message::IncrementMinutePressed => {
                let minutes = self.minutes + 1;
                if minutes == 60 {
                    self.hours += 1;
                    self.minutes = 0;
                } else {
                    self.minutes = minutes;
                }
            }
            Message::DecrementMinutePressed => {
                if self.minutes == 0 && self.hours != 0 {
                    self.hours -= 1;
                    self.minutes = 59;
                } else if self.minutes > 0 {
                    self.minutes -= 1;
                }
            }
            Message::ActivateTimer(b) => {
                self.timer = b;
            }
            Message::Sleep => {
                if self.hours > 0 || self.minutes > 0 {
                    if self.timer {
                        // sleep::after(self.hours * 60 + self.minutes)
                        println!("Should sleep after {}", self.hours * 60 + self.minutes);
                    } else {
                        println!("Should sleep at {}:{}", self.hours, self.minutes);
                        // sleep::at(self.hours, self.minutes)
                    }
                    self.active = true;
                }
            }
            Message::Cancel => {
                sleep::cancel();
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let top_row = Row::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .padding(10)
            .push(
                Button::new(&mut self.quicktime_thirty_min_button, Text::new("Add 30 minutes"))
                    .width(Length::FillPortion(1))
                    .on_press(Message::QuicktimeAddThirtyMinPressed)
            )
            .push(
                Button::new(&mut self.quicktime_one_hour_button, Text::new("Add 1 hour"))
                    .width(Length::FillPortion(1))
                    .on_press(Message::QuicktimeAddOneHourPressed))
            .push(
                Button::new(&mut self.quicktime_two_hours_button, Text::new("Add 2 hours"))
                    .width(Length::FillPortion(1))
                    .on_press(Message::QuicktimeAddTwoHoursPressed));
        let top_container = Container::new(top_row);

        let left_column = Column::new()
            .width(Length::FillPortion(10))
            .spacing(10)
            .align_items(Align::Center)
            .push(Text::new("hour:"))
            .push(Row::new().push(
                Button::new(&mut self.decrement_hour_button, Text::new("-"))
                    .on_press(Message::DecrementHourPressed)
                    .width(Length::Units(30))
            )
                .push(Text::new(self.hours.to_string()).size(25).width(Length::Units(50)))
                .push(Button::new(&mut self.increment_hour_button, Text::new("+"))
                    .on_press(Message::IncrementHourPressed)
                    .width(Length::Units(30)))
            )
            .push(Text::new("minute:"))
            .push(Row::new().push(
                Button::new(&mut self.decrement_minute_button, Text::new("-"))
                    .on_press(Message::DecrementMinutePressed)
                    .width(Length::Units(30))
            )
                .push(Text::new(self.minutes.to_string()).size(25).width(Length::Units(50)))
                .push(Button::new(&mut self.increment_minute_button, Text::new("+"))
                    .on_press(Message::IncrementMinutePressed)
                    .width(Length::Units(30))
                )
            )
            .push(
                Button::new(&mut self.sleep_button, Text::new("Sleep"))
                    .on_press(Message::Sleep)
                    .width(Length::Units(65))
            );
        let left_container = Container::new(left_column)
            .width(Length::FillPortion(1));


        let right_column = Column::new()
            .width(Length::FillPortion(1))
            .spacing(20)
            .align_items(Align::Center)
            .push(Checkbox::new(self.timer, "Timer mode", Message::ActivateTimer))
            .push(
                Text::new("TODO: get time left".to_string()).size(25)
            )
            .push(
                Button::new(&mut self.cancel_button, Text::new("Cancel"))
                    .on_press(Message::Cancel)
                    .width(Length::Units(67))
            );
        let right_container = Container::new(right_column)
            .width(Length::FillPortion(1));


        let main_column = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(top_container)
            .push(Row::new()
                .push(left_container)
                .push(right_container));

        Container::new(main_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}


mod style {}