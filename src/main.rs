use noncryptor::base64;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("Deux Ex Machina requires at least one command.");
        return;
    }

    let mut input_start = 2;

    match args[1].as_str() {
        "64" => {
            if args[2] == "-d" || args[2] == "--decode" {
                println!("{}", base64::decode(&args[3]));
            } else {
                if args[2] == "-e" || args[2] == "--encode" {
                    input_start += 1
                }
                let mut plain_text = String::new();
                for s in &args[input_start..args.len()] {
                    plain_text.push_str(s);
                    if args.len() > input_start {
                        plain_text.push(' ');
                    }
                }
                println!("{}", base64::encode(&plain_text))
            }
        }
        _ => println!("Command is unknown to Deus Ex Machina."),
    }
}
