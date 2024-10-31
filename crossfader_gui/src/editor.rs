use std::sync::Arc;

use nih_plug::{editor::Editor, prelude::GuiContext};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use style::{self};

use crate::XFaderParams;

pub(crate) fn default_state() -> Arc<IcedState> {
    return IcedState::from_size(180, 150);
}

pub(crate) fn create(
    params: Arc<XFaderParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<XFaderEditor>(editor_state, params)
}

struct XFaderEditor {
    params: Arc<XFaderParams>,
    context: Arc<dyn GuiContext>,
    // params
    fade_strength_state: nih_widgets::param_slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for XFaderEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<XFaderParams>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = Self {
            params,
            context,
            fade_strength_state: Default::default(),
        };
        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        return Command::none();
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .spacing(style::spacing::MD)
            .push(
                Text::new("X¹Fader")
                    .font(style::font::FIRA_BOLD)
                    .size(style::font::size::XL)
                    .color(style::color::BASE_CONTENT)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(style::spacing::MD.into()))
            .push(
                Text::new("Fade Strength")
                    .font(style::font::FIRA_REGULAR)
                    .size(style::font::size::MD)
                    .color(style::color::BASE_CONTENT),
            )
            .push(
                nih_widgets::ParamSlider::new(
                    &mut self.fade_strength_state,
                    &self.params.fade_strength,
                )
                .font(style::font::FIRA_REGULAR)
                .map(Message::ParamUpdate),
            )
            .into()
    }

    fn background_color(&self) -> Color {
        return style::color::BASE;
    }
}
