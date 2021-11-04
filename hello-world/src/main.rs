use std::env;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn help() {
    println!(
        "usage:
-h
    Print this message.
--list-hosts
    List available hosts and exit.
--host <host>"
    );
}

fn list_hosts(hosts: &Vec<cpal::HostId>) {
    println!("Hosts supported on this platform:\n  {:?}", cpal::ALL_HOSTS);
    println!("Available hosts:\n  {:?}", hosts);
}

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    let available_hosts = cpal::available_hosts();

    let mut host_id_to_use: Option<&cpal::HostId> = None;

    match args.len() {
        1 => {}
        2 => match args[1].as_str() {
            "--list-hosts" => {
                list_hosts(&available_hosts);
                return Ok(());
            }
            _ => {
                help();
                return Ok(());
            }
        },
        3 => match args[1].as_str() {
            "--host" => {
                host_id_to_use = available_hosts.iter().find(|host_id| {
                    host_id.name().to_lowercase() == args[2].as_str().to_lowercase()
                });

                if host_id_to_use.is_none() {
                    println!("That host could not be found or is not available.");
                    list_hosts(&available_hosts);
                    return Ok(());
                }
            }
            _ => {
                help();
                return Ok(());
            }
        },
        _ => {
            help();
            return Ok(());
        }
    }

    if host_id_to_use.is_none() {
        if available_hosts.len() < 1 {
            println!("No audio hosts available.");
            return Ok(());
        }
        host_id_to_use = Some(&available_hosts[0]);
    }

    let host = cpal::host_from_id(*host_id_to_use.unwrap())?;
    let device = host.default_output_device().unwrap();

    let channels = 2 as usize;
    let sample_rate = 44100;

    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % ((sample_rate as f32) * 10.0);
        [
            sin(sample_clock, sample_rate as f32, 440.0) * 0.2 * sin(sample_clock, sample_rate as f32, 1.7), // 1
            sin(sample_clock, sample_rate as f32, 660.0) * 0.2 * sin(sample_clock, sample_rate as f32, 0.21), // 5
            sin(sample_clock, sample_rate as f32, 880.0) * 0.2 * sin(sample_clock, sample_rate as f32, 0.55), // 8
            sin(sample_clock, sample_rate as f32, 840.0) * 0.2 * sin(sample_clock, sample_rate as f32, 0.257), // 7
            sin(sample_clock, sample_rate as f32, 495.0) * 0.2 * sin(sample_clock, sample_rate as f32, 0.3312), // 2
            sin(sample_clock, sample_rate as f32, 370.0) * 0.2 * sin(sample_clock, sample_rate as f32, 0.11), // -2
            sin(sample_clock, sample_rate as f32, 300.0) * 0.2 * sin(sample_clock, sample_rate as f32, 0.11113), // -4
        ]
        .iter()
        .fold(0f32, |acc, elem| elem + acc)
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        &device.default_output_config().unwrap().into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(10000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

fn sin(sample_clock: f32, sample_rate: f32, freq: f32) -> f32 {
    (sample_clock * freq * 2.0 * std::f32::consts::PI / sample_rate as f32).sin()
}
