//! Methods to interact with VST2 plugins

use crate::effects::*;
use crate::frame::*;
use crate::helpers::*;
use std::path::Path;
use std::sync::{Arc, Mutex};
use vst::host::HostBuffer;
use vst::host::{Host, PluginInstance, PluginLoader};
use vst::plugin::Plugin;

pub struct SimpleHost;
impl Host for SimpleHost {}

pub fn new_host() -> Arc<Mutex<SimpleHost>> {
    Arc::new(Mutex::new(SimpleHost))
}

pub fn load_plugin<P: AsRef<Path>>(
    path: P,
    host: &Arc<Mutex<SimpleHost>>,
) -> PluginLoader<SimpleHost> {
    PluginLoader::load(path.as_ref(), Arc::clone(host))
        .unwrap_or_else(|e| panic!("Failed to load plugin: {}", e))
}

pub struct VstPlugin {
    plugin: Mutex<PluginInstance>,
}
impl VstPlugin {
    pub fn new(loader: &mut PluginLoader<SimpleHost>, sample_rate: u32) -> Self {
        let mut instance = loader.instance().unwrap();
        instance.init();
        instance.set_sample_rate(sample_rate as f32);

        Self {
            plugin: Mutex::from(instance),
        }
    }

    /// Get `input.len()` samples out of `instance`
    /// Uses the default number of channels provided by the plugin
    ///
    /// Internally calls the plugins `process_f64` method
    pub fn process_samples(&self, input: Vec<Frame>) -> Vec<Frame> {
        let instance = self.plugin.lock().unwrap();

        let input_channels = instance.get_info().inputs as usize;
        let output_channels = instance.get_info().outputs as usize;
        drop(instance);

        self.process_samples_with_channels(input, input_channels, output_channels)
    }

    /// Get `input.len()` samples out of `instance`
    /// Uses the specified number of channels for input and output
    ///
    /// Internally calls the plugins `process_f64` method
    pub fn process_samples_with_channels(
        &self,
        input: Vec<Frame>,
        input_channels: usize,
        output_channels: usize,
    ) -> Vec<Frame> {
        let mut instance = self.plugin.lock().unwrap();

        let mut host_buffer: HostBuffer<f64> = HostBuffer::new(input_channels, output_channels);
        let length = input.len();

        let inputs = match input_channels {
            0 => vec![],
            1 => vec![input.into_mono()],
            _ => {
                let (left, right) = input.split();
                let mut vec = vec![left, right];
                vec.append(&mut vec![vec![0.0; length]; input_channels - 2]);
                vec
            }
        };
        let mut outputs = vec![vec![0.0; length]; output_channels];
        let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);
        instance.process_f64(&mut audio_buffer);

        let output = audio_buffer.split().1;
        match output.len() {
            2 => output
                .get(0)
                .iter()
                .zip(output.get(1).iter())
                .map(|(a, b)| Frame::new(*a, *b))
                .collect(),
            _ => output.get(0).iter().map(|a| Frame::mono(*a)).collect(),
        }
    }

    /// Same as it's non-f32 alternative, but calls `process` instead of calling `process_f64`
    pub fn process_samples_f32(&self, input: Vec<Frame>) -> Vec<Frame> {
        let instance = self.plugin.lock().unwrap();

        let input_channels = instance.get_info().inputs as usize;
        let output_channels = instance.get_info().outputs as usize;
        drop(instance);

        self.process_samples_with_channels_f32(input, input_channels, output_channels)
    }

    /// Same as it's non-f32 alternative, but calls `process` instead of calling `process_f64`
    pub fn process_samples_with_channels_f32(
        &self,
        input: Vec<Frame>,
        input_channels: usize,
        output_channels: usize,
    ) -> Vec<Frame> {
        let mut instance = self.plugin.lock().unwrap();

        let mut host_buffer: HostBuffer<f32> = HostBuffer::new(input_channels, output_channels);

        let length = input.len();

        let inputs = match input_channels {
            0 => vec![],
            1 => vec![input.into_mono().iter().map(|a| *a as f32).collect()],
            _ => {
                let (left, right) = input.split();
                let mut vec = vec![
                    left.iter().map(|a| *a as f32).collect(),
                    right.iter().map(|a| *a as f32).collect(),
                ];
                vec.append(&mut vec![vec![0.0; length]; input_channels - 2]);
                vec
            }
        };
        let mut outputs = vec![vec![0.0; length]; output_channels];
        let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);
        instance.process(&mut audio_buffer);

        let output = audio_buffer.split().1;
        match output.len() {
            2 => output
                .get(0)
                .iter()
                .zip(output.get(1).iter())
                .map(|(a, b)| Frame::new(*a as f64, *b as f64))
                .collect(),
            _ => output
                .get(0)
                .iter()
                .map(|a| Frame::mono(*a as f64))
                .collect(),
        }
    }
}

impl Effect for VstPlugin {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        self.process_samples(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO Add tests for other OSes
    #[cfg(target_os = "macos")]
    #[test]
    fn can_open_test_plugin() {
        let host = new_host();
        let mut loader = load_plugin(
            "assets/vsts/TestPlugin.vst/Contents/MacOS/TestPlugin",
            &host,
        );
        let plugin = VstPlugin::new(&mut loader, 44_100);
        let instance = plugin.plugin.lock().unwrap();

        let info = instance.get_info();

        assert_eq!("TestSynth".to_string(), info.name);
        assert_eq!("Dawremi".to_string(), info.vendor);
        assert_eq!(6667, info.unique_id);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn can_get_samples_from_test_plugin() {
        let host = new_host();
        let mut loader = load_plugin(
            "assets/vsts/TestPlugin.vst/Contents/MacOS/TestPlugin",
            &host,
        );
        let instance = VstPlugin::new(&mut loader, 44_100);

        let audio = instance.process_samples(vec![Frame::mono(0.); 1000]);

        assert_eq!(1000, audio.len());

        // TestPlugin returns a sine wave at a specific frequency
        // Here we check that we get a predefined set of values
        assert!(audio[0].left < 0.0000001);
        assert!(audio[0].right < 0.0000001);

        assert!(audio[1].left > 0.000001);
        assert!(audio[1].right > 0.000001);

        assert!(audio[999].left < -0.0201566071);
        assert!(audio[999].left > -0.0201566072);
        assert!(audio[999].right < -0.0201566071);
        assert!(audio[999].right > -0.0201566072);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn can_use_plugin_as_effect() {
        let host = new_host();
        let mut loader = load_plugin(
            "assets/vsts/TestPlugin.vst/Contents/MacOS/TestPlugin",
            &host,
        );
        let instance = VstPlugin::new(&mut loader, 44_100);

        let audio = vec![Frame::mono(0.); 1000].effect(&instance);

        assert_eq!(1000, audio.len());

        // TestPlugin returns a sine wave at a specific frequency
        // Here we check that we get a predefined set of values
        assert!(audio[0].left < 0.0000001);
        assert!(audio[0].right < 0.0000001);

        assert!(audio[1].left > 0.000001);
        assert!(audio[1].right > 0.000001);

        assert!(audio[999].left < -0.0201566071);
        assert!(audio[999].left > -0.0201566072);
        assert!(audio[999].right < -0.0201566071);
        assert!(audio[999].right > -0.0201566072);
    }
}
