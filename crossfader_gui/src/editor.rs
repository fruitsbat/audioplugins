use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::XFaderParams;

#[derive(Lens)]
struct Data {
    params: Arc<XFaderParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    return ViziaState::new(|| (200, 150));
}

pub(crate) fn create(
    params: Arc<XFaderParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "X¹Fader")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_size(style::font::size::XL);

            Label::new(cx, "Fade Strength");
            ParamSlider::new(cx, Data::params, |params| &params.fade_strength);
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}
