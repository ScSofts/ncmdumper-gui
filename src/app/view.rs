use iced::{Alignment, Element, Padding};
use iced::border::Radius;
use iced::widget::{button, column, progress_bar, row, Row, svg, Svg, text, text_input, tooltip};
use iced::widget::text::Shaping;
use iced::widget::tooltip::Position;

use crate::app::{Application, Message, res, State};
use crate::app::style::{ButtonBorder, ButtonColor};

const INPUT_PATH: &str = "输入路径或选择文件(夹)";
const SELECT_PATH: &str = "点击以选择路径";
const OUTPUT_PATH: &str = "输入保存路径或选择文件(夹)";
const START_CONVERT: &str = "开始转换";

fn load_svg(svg_source: &'static [u8]) -> Svg {
    let handle = svg::Handle::from_memory(svg_source);
    svg(handle).into()
}

pub fn path_input(this: &Application) -> Row<'static, Message> {
    let button_select_background = ButtonColor::new(iced::Color::from_rgb8(0x3e, 0x3f, 0x42));
    let tooltip_border = ButtonBorder::new(iced::Color::from_rgb8(0xee, 0xef, 0xf2), 1.0, Radius::default());
    let tooltip_color = iced::theme::Text::Color(iced::Color::from_rgb8(0xde, 0xdf, 0xe2));

    let mut input = text_input(
        INPUT_PATH,
        this.input_path.as_str(),
    )
        .width(360);

    if this.state == State::Idle {
        input = input.on_input(Message::InputPath);
    }

    let path_input = row([
        input.into(),
        tooltip(button(load_svg(res::FOLDER_ICON))
                    .style(button_select_background)
                    .width(40)
                    .height(30)
                    .on_press_maybe(if this.state == State::Idle {
                        Some(Message::SelectPath)
                    } else {
                        None
                    }),
                button(
                    text(SELECT_PATH)
                        .shaping(Shaping::Advanced)
                        .style(tooltip_color)
                )
                    .style(tooltip_border),
                Position::Bottom,
        ).into()
    ])
        .spacing(5)
        .into();

    path_input
}


pub fn path_output(this: &Application) -> Row<'static, Message> {
    let button_select_background = ButtonColor::new(iced::Color::from_rgb8(0x3e, 0x3f, 0x42));
    let tooltip_border = ButtonBorder::new(iced::Color::from_rgb8(0xee, 0xef, 0xf2), 1.0, Radius::default());
    let tooltip_color = iced::theme::Text::Color(iced::Color::from_rgb8(0xde, 0xdf, 0xe2));

    let mut input = text_input(
        OUTPUT_PATH,
        this.output_path.as_str(),
    )
        .width(360);

    if this.state == State::Idle {
        input = input.on_input(Message::OutputPath);
    }

    let path_output = row([
        input.into(),
        tooltip(button(load_svg(res::FOLDER_ICON))
                    .style(button_select_background)
                    .width(40)
                    .height(30)
                    .on_press_maybe(if this.state == State::Idle {
                        Some(Message::SelectOutputPath)
                    } else {
                        None
                    }),
                button(
                    text(SELECT_PATH)
                        .shaping(Shaping::Advanced)
                        .style(tooltip_color)
                )
                    .style(tooltip_border),
                Position::Bottom,
        ).into()
    ])
        .spacing(5)
        .into();

    path_output
}

pub(crate) fn view(this: &Application) -> Element<'_, Message> {
    let text_color = iced::theme::Text::Color(iced::Color::from_rgb8(0xde, 0xdf, 0xe2));

    column([
        path_input(this).into(),
        path_output(this).into(),
        button(
            text(START_CONVERT)
                .shaping(Shaping::Advanced)
                .style(text_color)
                .size(26)
        )
            .padding(Padding::from([20, 25]))
            .on_press_maybe(if this.state == State::Idle {
                Some(Message::Start)
            } else {
                None
            })
            .style(ButtonColor::new(iced::Color::from_rgb8(0x3e, 0x3f, 0x42)))
            .width(160)
            .height(80)
            .into(),
        progress_bar(0.0..=1.0, this.progress)
            .width(400)
            .height(20)
            .into()
    ])
        .spacing(40)
        .width(600)
        .height(480)
        .align_items(Alignment::Center)
        .padding(100)
        .into()
}
