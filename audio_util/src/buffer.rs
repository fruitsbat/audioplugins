use nih_plug::prelude::AuxiliaryBuffers;

/// checks sidechain sample at the position the main sample is
/// just returns zero when there is an error
pub fn get_sample_at_position(
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
