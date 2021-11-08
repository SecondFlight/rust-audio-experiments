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
    let path = Path::new("C:/Program Files/VST Plugins/OTT_x64.dll");

    let mut loader = PluginLoader::load(path, host.clone()).unwrap();
    let mut instance = loader.instance().unwrap();

    println!("{:?}", instance.get_info());

    instance.init();
    println!("Initialized instance!");

    println!("Closing instance...");
    // Not necessary as the instance is shut down when it goes out of scope anyway.
    // drop(instance);
}