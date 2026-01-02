use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    address: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(short, long, default_value_t = false)]
    show_output: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut tasks: Vec<tokio::task::JoinHandle<Option<String>>> = vec![];

    for i in 0..args.count {
        println!("Generate process {}/{}...", i + 1, args.count);
        tasks.push(tokio::spawn(request(args.address.clone())));
    }

    let (mut count, len) = (0, tasks.len());
    for task in tasks {
        count += 1;
        println!("Enumerating result {count}/{}", len);
        match task.await {
            Ok(s) => {
                if let Some(answer) = s {
                    if args.show_output {
                        println!("{answer}");
                    }
                } else {
                    eprintln!("Request error");
                    return;
                }
            }
            Err(e) => {
                eprintln!("Some postrun error occured: {e}");
                return;
            }
        };
    }
}

async fn request(address: String) -> Option<String> {
    match reqwest::get(address).await {
        Ok(s) => match s.text().await {
            Ok(t) => Some(t),
            Err(e) => {
                eprintln!("Error unpacking text: {e}");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Request failed: {e}");
            std::process::exit(1);
        }
    }
}
