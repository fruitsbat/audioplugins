use nih_plug::prelude::*;
use std::sync::Arc;

struct XYFade {
    params: Arc<ConstantPowerCrossfadeParams>,
}

impl Default for XYFade {
    fn default() -> Self {
        Self {
            params: Arc::new(ConstantPowerCrossfadeParams::default()),
        }
    }
}

#[derive(Params)]
struct ConstantPowerCrossfadeParams {
    #[id = "X"]
    pub x_slider: FloatParam,
    #[id = "Y"]
    pub y_slider: FloatParam,
}

impl Default for ConstantPowerCrossfadeParams {
    fn default() -> Self {
        Self {
            x_slider: FloatParam::new("X", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            y_slider: FloatParam::new("Y", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
        }
    }
}

impl Plugin for XYFade {
    // metadata
    const NAME: &'static str = "XY Fader";
    const VENDOR: &'static str = "zoe bat";
    const URL: &'static str = "https://zoe.kittycat.homes";
    const EMAIL: &'static str = "zoe@kittycat.homes";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        // stereo
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[
                new_nonzero_u32(2),
                new_nonzero_u32(2),
                new_nonzero_u32(2),
                new_nonzero_u32(2),
                new_nonzero_u32(2),
                new_nonzero_u32(2),
                new_nonzero_u32(2),
                new_nonzero_u32(2),
            ],
            aux_output_ports: &[],

            names: PortNames {
                layout: Some("Stereo"),
                main_input: Some("[0, 0, 0]"),
                main_output: Some("Out"),
                aux_inputs: &["[1, 0, 0]", "[0, 1, 0]", "[1, 1, 0]"],
                aux_outputs: &[],
            },
        },
    ];

    type SysExMessage = ();

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let x_mix_value = self.params.x_slider.smoothed.next();
        let x_low_volume = slider_low_audio_mix_value(x_mix_value);
        let x_high_volume = slider_high_audio_mix_value(x_low_volume);

        let y_mix_value = self.params.y_slider.smoothed.next();
        let y_low_volume = slider_low_audio_mix_value(y_mix_value);
        let y_high_volume = slider_high_audio_mix_value(y_mix_value);

        // apply to main audio input
        for (sample_index, channel_samples) in buffer.iter_samples().enumerate() {
            for (channel_index, sample) in channel_samples.into_iter().enumerate() {
                let sample_0_0 = *sample;
                let sample_1_0 =
                    get_sidechain_value_for_main_sample(channel_index, sample_index, aux, 1);
                let sample_0_1 =
                    get_sidechain_value_for_main_sample(channel_index, sample_index, aux, 0);
                let sample_1_1 =
                    get_sidechain_value_for_main_sample(channel_index, sample_index, aux, 2);

                let low_y = (sample_0_0 * x_low_volume) + (sample_1_0 * x_high_volume);
                let high_y = (sample_0_1 * x_low_volume) + (sample_1_1 * x_high_volume);

                *sample = (low_y * y_low_volume) + (high_y * y_high_volume);
            }
        }

        ProcessStatus::Normal
    }
}

/// checks sidechain sample at the position the main sample is
/// just returns zero when there is an error
fn get_sidechain_value_for_main_sample(
    channel_index: usize,
    sample_index: usize,
    aux: &mut AuxiliaryBuffers,
    aux_in_selection: usize,
) -> f32 {
    if let Some(buffer) = aux.inputs.get(aux_in_selection) {
        let buffer = buffer.as_slice_immutable();
        if let Some(sample) = buffer.get(channel_index) {
            if let Some(channel) = sample.get(sample_index) {
                return *channel;
            }
        }
    }

    0.0
}

fn slider_low_audio_mix_value(mix_value: f32) -> f32 {
    f32::sqrt(mix_value)
}

fn slider_high_audio_mix_value(mix_value: f32) -> f32 {
    f32::sqrt(1.0 - mix_value)
}

impl ClapPlugin for XYFade {
    const CLAP_ID: &'static str = "fruitsuite.xy_fader ";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Two dimensional crossfading!");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = Some("https://zoe.kittycat.homes");
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Stereo,
        ClapFeature::Utility,
        ClapFeature::Custom("Crossfade"),
        ClapFeature::AudioEffect,
    ];
}

impl Vst3Plugin for XYFade {
    const VST3_CLASS_ID: [u8; 16] = *b"fruit.XYFader000";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Stereo,
        Vst3SubCategory::Tools,
    ];
}

nih_export_clap!(XYFade);
nih_export_vst3!(XYFade);
