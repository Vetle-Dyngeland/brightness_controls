use std::{io::stdin, process::Command};

fn main() {
    let input = std::env::args()
        .collect::<Vec<String>>()
        .iter()
        .filter_map(|s| {
            let num = match s.trim().parse::<f32>() {
                Ok(n) => n,
                Err(_) => return None,
            };

            Some(num)
        })
        .collect::<Vec<f32>>();

    let devices = match get_devices() {
        Ok(devices) => devices,
        Err(msg) => panic!("Could not get devices. Message: \n{msg}"),
    };

    let selected = if input.len() >= 1 {
        if input[0] >= 0f32 && (input[0] as usize) < devices.len() {
            input[0] as usize
        } else {
            get_selected(&devices)
                .unwrap_or_else(|err| panic!("Could not get selected device index. Error: {err}"))
        }
    } else {
        get_selected(&devices)
            .unwrap_or_else(|err| panic!("Could not get selected device index. Error: {err}"))
    };

    println!("What should the brightness value be? (min: 0.3, max: 4.9, default: 1.0)");
    let brightness = if input.len() >= 2 {
        if input[1] <= 4.9 && input[1] >= 0.3 {
            input[1].to_string()
        } else {
            get_brightness()
        }
    } else {
        get_brightness()
    }
    .trim().to_string();

    match Command::new("xrandr")
        .arg("--output")
        .arg(devices[selected].clone())
        .arg("--brightness")
        .arg(brightness.clone())
        .output()
    {
        Ok(_) => println!("Brightness set successfully"),
        Err(err) => {
            panic!(
                "Could not set brightness of device {} to {brightness}. Error: \n{}",
                devices[selected],
                err.to_string()
            );
        }
    }
}

fn get_brightness() -> String {
    get_input(
        |s| {
            let s = s.trim();
            let f = match s.parse::<f32>() {
                Ok(num) => num,
                Err(msg) => {
                    println!("Could not parse {s} into f32. Message: {}", msg.to_string());
                    return false;
                }
            };

            f >= 0.3f32 && f <= 4.9f32
        },
        None,
    )
}

fn get_selected(devices: &Vec<String>) -> Result<usize, String> {
    println!("Type index of device below");
    for (i, device) in devices.iter().enumerate() {
        println!("{i}: {device}")
    }
    let selected = get_input(
        |s| {
            let c = match s.trim().chars().nth(0) {
                Some(c) => c,
                None => return false,
            };
            c.is_ascii_digit() && c.to_digit(10).is_some_and(|i| i < devices.len() as u32)
        },
        None,
    );

    match selected.trim().parse::<usize>() {
        Ok(val) => Ok(val),
        Err(err) => Err(err.to_string()),
    }
}

fn get_devices() -> Result<Vec<String>, String> {
    let output = match Command::new("xrandr").output() {
        Ok(output) => output,
        Err(err) => {
            return Err(format!(
                "Could not get output of xrandr. Error: \n{}",
                err.to_string()
            ))
        }
    };

    let output = match String::from_utf8(output.stdout) {
        Ok(string) => string,
        Err(err) => {
            return Err(format!(
                "Could not convert output to utf8. Error: \n{}",
                err.to_string()
            ))
        }
    };

    let output = output
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .filter(|s| {
            !s.starts_with(" ")
                && match s.split(' ').nth(1) {
                    Some(s) => s.trim() == "connected",
                    None => false,
                }
        })
        .map(|s| s.to_owned().to_owned())
        .map(|s| {
            s.split_once(" ")
                .expect("String does not contain any spaces!")
                .0
                .to_string()
        })
        .collect();

    Ok(output)
}

fn get_input<F>(req: F, error_message: Option<String>) -> String
where
    F: Fn(&str) -> bool,
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    if req(&input) {
        input
    } else {
        println!(
            "{}",
            match error_message.clone() {
                Some(msg) => msg,
                None => "Input did not meet requirenments, try again".to_string(),
            }
        );
        get_input(req, error_message)
    }
}
