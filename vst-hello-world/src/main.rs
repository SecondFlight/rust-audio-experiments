extern crate vst;

use std::sync::{Arc, Mutex};
use std::path::Path;

use vst::host::{Host, PluginLoader};
use vst::plugin::Plugin;

struct SampleHost;

impl Host for SampleHost {
    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }
}

fn main() {
    let host = Arc::new(Mutex::new(SampleHost));
    let path = Path::new("C:/Program Files/VstPlugins/OTT_x64.dll");

    let mut loader = PluginLoader::load(path, host.clone()).unwrap();
    let mut instance = loader.instance().unwrap();
    let info = instance.get_info();

    println!("{:?}", info);

    let parameter_count = info.parameters;

    for i in 0..parameter_count {
        println!("{}", instance.get_parameter_object().get_parameter_name(i));
    }

    instance.init();
    // println!("Initialized instance!");

    // println!("Closing instance...");
    // Not necessary as the instance is shut down when it goes out of scope anyway.
    // drop(instance);
}