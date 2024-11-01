use nih_plug::prelude::*;
use std::sync::Arc;

struct XFade {
    params: Arc<ConstantPowerCrossfadeParams>,
}

impl Default for XFade {
    fn default() -> Self {
        Self {
            params: Arc::new(ConstantPowerCrossfadeParams::default()),
        }
    }
}

#[derive(Params)]
struct ConstantPowerCrossfadeParams {
    #[id = "8eef0a45-7715-4714-845c-1d2dd0bf7c52"]
    pub main_side_mix: FloatParam,
}

impl Default for ConstantPowerCrossfadeParams {
    fn default() -> Self {
        Self {
            main_side_mix: FloatParam::new("X Mix", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(10.0))
                .with_step_size(0.0001),
        }
    }
}

impl Plugin for XFade {
    // metadata
    const NAME: &'static str = "X Fader";
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
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mix_value = self.params.main_side_mix.smoothed.next();
        let main_audio_mix = main_audio_mix_value(mix_value);
        let sidechain_mix = sidechain_audio_mix_value(mix_value);

        // apply to main audio input
        for (sample_index, channel_samples) in buffer.iter_samples().enumerate() {
            for (channel_index, sample) in channel_samples.into_iter().enumerate() {
                *sample *= main_audio_mix;
                *sample += get_sidechain_value_for_main_sample(channel_index, sample_index, aux)
                    * sidechain_mix;
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
) -> f32 {
    if let Some(buffer) = aux.inputs.first() {
        let buffer = buffer.as_slice_immutable();
        if let Some(sample) = buffer.get(channel_index) {
            if let Some(channel) = sample.get(sample_index) {
                return *channel;
            }
        }
    }

    0.0
}

fn main_audio_mix_value(mix_value: f32) -> f32 {
    f32::sqrt(mix_value)
}

fn sidechain_audio_mix_value(mix_value: f32) -> f32 {
    f32::sqrt(1.0 - mix_value)
}

impl ClapPlugin for XFade {
    const CLAP_ID: &'static str = "fruitsuite.x_fader ";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("DJ style constant power crossfading plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = Some("https://zoe.kittycat.homes");
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Stereo,
        ClapFeature::Utility,
        ClapFeature::Custom("Crossfade"),
        ClapFeature::AudioEffect,
    ];
}

impl Vst3Plugin for XFade {
    const VST3_CLASS_ID: [u8; 16] = *b"Vx2f3D7k!a@cJS8H";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Stereo,
        Vst3SubCategory::Tools,
    ];
}

nih_export_clap!(XFade);
nih_export_vst3!(XFade);
