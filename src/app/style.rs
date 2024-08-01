use iced::{Border, Color, Theme};
use iced::border::Radius;
use iced::widget::button;
use iced::widget::button::Appearance;

pub struct ButtonColor {
    color: Color,
}


impl button::StyleSheet for ButtonColor {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) ->  Appearance{
        Appearance {
            background: Some(iced::Background::Color(self.color)),
            ..Default::default()
        }
    }
}

impl ButtonColor{
    pub(crate) fn new(color: Color) -> iced::theme::Button{
        iced::theme::Button::Custom(
            Box::new(
                Self{
                    color
                }
            )
        )
    }
}

pub struct ButtonBorder{
    color: Color,
    width: f32,
    radius: Radius
}

impl button::StyleSheet for ButtonBorder {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) ->  Appearance{
        Appearance {
            border: Border{
                color: self.color,
                width: self.width,
                radius: self.radius,
            },
            ..Default::default()
        }
    }
}

impl ButtonBorder{
    pub(crate) fn new(color: Color, width: f32, radius: Radius) -> iced::theme::Button{
        iced::theme::Button::Custom(
            Box::new(
                Self{
                    color,
                    width,
                    radius
                }
            )
        )
    }
}

