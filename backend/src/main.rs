use clap::{App, Arg};

#[cfg(feature = "api")]
mod api; // API code in api.rs

#[cfg(feature = "cli")]
mod cli; // CLI code in cli.rs


#[tokio::main]
async fn main() {
    let matches = App::new("QueryHive 🧠✨ CLI and API")
        .version("1.0")
        .about("QueryHive 🧠✨ CLI and API example")
        .arg(Arg::new("mode")
            .short('m')
            .long("mode")
            .takes_value(true)
            .help("Specify the mode (cli or api)"))
        .get_matches();

    let mode = matches.value_of("mode").unwrap_or("api");

    if mode == "cli" {
        // Run CLI application
        println!("Running in CLI mode");
        cli::run(); // CLI functionality
    } else {
        // Run Web API
        println!("Running in API mode");
        api::run(); // Web API functionality
    }
}
