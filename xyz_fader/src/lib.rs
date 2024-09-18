use nih_plug::prelude::*;
use std::sync::Arc;

struct XYZFade {
    params: Arc<ConstantPowerCrossfadeParams>,
}

impl Default for XYZFade {
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
    #[id = "Z"]
    pub z_slider: FloatParam,
}

impl Default for ConstantPowerCrossfadeParams {
    fn default() -> Self {
        Self {
            x_slider: FloatParam::new("X", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(10.0)),
            y_slider: FloatParam::new("Y", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(10.0)),
            z_slider: FloatParam::new("Z", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(10.0)),
        }
    }
}

impl Plugin for XYZFade {
    // metadata
    const NAME: &'static str = "XYZ Fader";
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
            ],
            aux_output_ports: &[],

            names: PortNames {
                layout: Some("Stereo"),
                main_input: Some("[0, 0, 0]"),
                main_output: Some("Out"),
                aux_inputs: &[
                    "[1, 0, 0]",
                    "[0, 0, 1]",
                    "[1, 0, 1]",
                    "[0, 1, 0]",
                    "[1, 1, 0]",
                    "[0, 1, 1]",
                    "[1, 1, 1]",
                ],
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

        let z_mix_value = self.params.z_slider.smoothed.next();
        let z_low_volume = slider_low_audio_mix_value(z_mix_value);
        let z_high_value = slider_high_audio_mix_value(z_mix_value);

        // apply to main audio input
        for (sample_index, channel_samples) in buffer.iter_samples().enumerate() {
            for (channel_index, sample) in channel_samples.into_iter().enumerate() {
                let sample_0_0_0 = *sample;
                let sample_1_0_0 =
                    get_sidechain_value_for_main_sample(channel_index, sample_index, aux, 0);
                let mix_x_1 = (sample_0_0_0 * x_low_volume) + (sample_1_0_0 * x_high_volume);

                let sample_0_0_1 =
                    get_sidechain_value_for_main_sample(channel_index, sample_index, aux, 1);
                let sample_1_0_1 =
                    get_sidechain_value_for_main_sample(channel_index, sample_index, aux, 2);
                let mix_x_2 = (sample_0_0_1 * x_low_volume) + (sample_1_0_1 * x_high_volume);

                let sample_0_1_0 =
                    get_sidechain_value_for_main_sample(channel_index, sample_index, aux, 3);
                let sample_1_1_0 =
                    get_sidechain_value_for_main_sample(channel_index, sample_index, aux, 4);
                let mix_x_3 = (sample_0_1_0 * x_low_volume) + (sample_1_1_0 * x_high_volume);

                let sample_0_1_1 =
                    get_sidechain_value_for_main_sample(channel_index, sample_index, aux, 5);
                let sample_1_1_1 =
                    get_sidechain_value_for_main_sample(channel_index, sample_index, aux, 6);
                let mix_x_4 = (sample_0_1_1 * x_low_volume) + (sample_1_1_1 * x_high_volume);

                let mix_y_1 = (mix_x_1 * y_low_volume) + (mix_x_2 * y_high_volume);
                let mix_y_2 = (mix_x_3 * y_low_volume) + (mix_x_4 * y_high_volume);

                *sample = (mix_y_1 * z_low_volume) + (mix_y_2 * z_high_value);
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

impl ClapPlugin for XYZFade {
    const CLAP_ID: &'static str = "fruitsbat.xyz_fader ";
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

impl Vst3Plugin for XYZFade {
    const VST3_CLASS_ID: [u8; 16] = *b"fruit.XYZFader00";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Stereo,
        Vst3SubCategory::Tools,
    ];
}

nih_export_clap!(XYZFade);
nih_export_vst3!(XYZFade);
