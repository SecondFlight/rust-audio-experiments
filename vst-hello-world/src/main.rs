// cspell:ignore HINSTANCE HWND libloaderapi lpfn lpsz minwindef OVERLAPPEDWINDOW winapi winuser WNDCLASSW

use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::prelude::OsStrExt;
use std::path::Path;
use std::ptr;
use std::sync::{Arc, Mutex};

use vst::host::{Host, PluginLoader};
use vst::plugin::Plugin;
use winapi::shared::minwindef::HINSTANCE__;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, RegisterClassW, ShowWindow, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE};

struct SampleHost;

impl Host for SampleHost {
    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }
}

fn win32_string( value : &str ) -> Vec<u16> {
    OsStr::new( value ).encode_wide().chain( once( 0 ) ).collect()
}

pub const WINDOW_CLASS_NAME: &str = "plugin_editor_class";

fn main() {
    let host = Arc::new(Mutex::new(SampleHost));
    let path = Path::new("C:/Program Files/VstPlugins/OTT_x64.dll");

    let mut loader = PluginLoader::load(path, host.clone()).unwrap();
    let mut instance = loader.instance().unwrap();
    let info = instance.get_info();

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
        let window_class = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some( DefWindowProcW ),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as *mut HINSTANCE__,
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
            lpszClassName: win32_string(WINDOW_CLASS_NAME).as_ptr(),
        };
        let class_atom = RegisterClassW(&window_class);
        if class_atom == 0 {
            panic!("Error registering window class");
        }
        let hinstance = GetModuleHandleW( ptr::null_mut() );
        parent = CreateWindowExW(
            0,
            win32_string(WINDOW_CLASS_NAME).as_ptr(),
            win32_string("editor").as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            0,
            0,
            size.0,
            size.1,
            ptr::null_mut(),
            ptr::null_mut(),
            hinstance,
            ptr::null_mut(),
        );
        ShowWindow(parent, 1);
    }
    let successful = editor.open(parent as *mut std::ffi::c_void);
    
    println!("could open editor with provided window: {}", successful);

    instance.resume();
    
    std::thread::sleep(std::time::Duration::from_millis(10000));

    println!("Closing instance...");
    // Not necessary as the instance is shut down when it goes out of scope anyway.
    // drop(instance);
}
