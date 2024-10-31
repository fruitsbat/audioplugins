use nih_plug::{
    params::{FloatParam, Params},
    plugin::Plugin,
    prelude::*,
};
use nih_plug_vizia::ViziaState;
use std::sync::Arc;
mod editor;
use audio_util::{self, buffer::get_sample_at_position, crossfade};

struct XFader {
    params: Arc<XFaderParams>,
}

impl Default for XFader {
    fn default() -> Self {
        Self {
            params: Arc::new(XFaderParams::default()),
        }
    }
}

#[derive(Params)]
pub struct XFaderParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[id = "53c56370-01c5-4820-a977-1dec2eef1af3"]
    pub fade_strength: FloatParam,
}

impl Default for XFaderParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            fade_strength: FloatParam::new(
                "Fade Strength",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(0.5))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(2)),
        }
    }
}

impl Plugin for XFader {
    const NAME: &'static str = "X¹Fader";
    const VENDOR: &'static str = "kittycat.homes";
    const URL: &'static str = "https://github.com/fruitsbat/audioplugins";
    const EMAIL: &'static str = "zoe@kittycat.homes";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        // stereo
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[new_nonzero_u32(2)],
            aux_output_ports: &[],

            names: PortNames {
                layout: Some("Stereo"),
                main_input: Some("Main"),
                main_output: Some("Out"),
                aux_inputs: &["Sidechain"],
                aux_outputs: &[],
            },
        },
    ];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        return self.params.clone();
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn process(
        &mut self,
        buffer: &mut nih_plug::prelude::Buffer,
        aux: &mut nih_plug::prelude::AuxiliaryBuffers,
        _context: &mut impl nih_plug::prelude::ProcessContext<Self>,
    ) -> nih_plug::prelude::ProcessStatus {
        {
            let linear_crossfade_power = self.params.fade_strength.smoothed.next();
            let constant_power_crossfade = crossfade::constant_power(linear_crossfade_power);
            let x0_fade_strength = constant_power_crossfade;
            let x1_fade_strength = 1.0 - constant_power_crossfade;

            // apply to main audio input
            for (sample_index, channel_samples) in buffer.iter_samples().enumerate() {
                for (channel_index, sample) in channel_samples.into_iter().enumerate() {
                    *sample *= x0_fade_strength;
                    *sample +=
                        get_sample_at_position(channel_index, sample_index, aux) * x1_fade_strength;
                }
            }

            ProcessStatus::Normal
        }
    }
}

impl ClapPlugin for XFader {
    const CLAP_ID: &'static str = "1b06d8ac-4c9c-4aef-a105-16bcb76d989b";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("DJ Style Crossfader");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Utility, ClapFeature::Stereo];
}

nih_export_clap!(XFader);
