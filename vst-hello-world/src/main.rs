extern crate vst;

use std::path::Path;
use std::sync::{Arc, Mutex};

use vst::host::{Host, PluginLoader};
use vst::plugin::Plugin;
use winapi::shared::minwindef::HINSTANCE__;
use winapi::shared::windef::{HMENU__};
use winapi::um::winuser::HWND_DESKTOP;

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
    println!("Initialized instance!");

    let mut editor = instance.get_editor().unwrap();
    let size = editor.size();

    // this is very gross
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexa
    let parent;
    unsafe {
        parent = winapi::um::winuser::CreateWindowExW(
            0,
            "STATIC".as_bytes().as_ptr() as *const u16,
            "editor".as_bytes().as_ptr() as *const u16,
            0x10000000,
            0,
            0,
            size.0,
            size.1,
            HWND_DESKTOP,
            0 as *mut HMENU__,
            0 as *mut HINSTANCE__,
            0 as *mut winapi::ctypes::c_void,
        );
        winapi::um::winuser::ShowWindow(parent, 1);
    }
    let successful = editor.open(parent as *mut std::ffi::c_void);
    
    println!("could open editor with provided window: {}", successful);

    std::thread::sleep(std::time::Duration::from_millis(10000));

    println!("Closing instance...");
    // Not necessary as the instance is shut down when it goes out of scope anyway.
    // drop(instance);
}
