use notify_rust::Notification;
use clap::{App, Arg};

const DEFAULT_MSG: &str = "\t<!>";
const DEFAULT_TIMEOUT: i32 = 3000;

struct Besked {
    message: String,
    timeout: i32
}

fn besked() -> Besked {
    let message = DEFAULT_MSG.to_string();
    let timeout = DEFAULT_TIMEOUT;
    let besked = Besked {
        message,
        timeout
    };
    besked
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time = &DEFAULT_TIMEOUT.to_string();
    let matches = App::new("sc-notify")
        .version("jelly!")
        .author("Niklas Adam <salkinmada@protonmail.com>")
        .about("notifications from sclang")
        .arg(
            Arg::with_name("message")
            .short("m")
            .long("message")
            .value_name("STRING")
            .help("notification body")
            .required(false)
            .takes_value(true)
        )
        .arg(
            Arg::with_name("timeout")
            .short("t")
            .long("timeout")
            .value_name("INTEGER")
            .default_value(time)
            .help("life time in ms  (0=inf)")
            .required(false)
            .takes_value(true)
        )
        .get_matches();

    let mut notification = Notification::new();
    let mut bsk = besked();
    
    if let Some(m) = matches.value_of("message") {
        bsk.message = m.to_string()
    }

    if let Some(timeout_string) = matches.value_of("timeout") {
        if let Ok(timeout) = timeout_string.parse::<i32>() {
            bsk.timeout = timeout;
        } else {
            println!(
                "sc-notify can't parse timeout {:?}, use an integer",
                timeout_string
            );
        }
    }

        notification.summary("sclang\n").icon("supercollider")
        .body(&bsk.message)
        .timeout(bsk.timeout)
        .show()?;
    Ok(())
}
