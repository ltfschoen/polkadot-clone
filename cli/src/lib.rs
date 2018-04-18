extern crate clap;
#[macro_use] extern crate log;
extern crate env_logger;

use clap::{Arg, App, SubCommand};

pub fn main() {
	let matches = App::new("Polkadot-clone")
		.version("0.1")
		.author("Luke S. <ltfschoen@gmail.com>")
		.about("Implementation Polkadot node in Rust")
		.arg(Arg::with_name("log")
			.short("l")
			.value_name("LOG")
			.help("Sets logging.")
			.required(false)
			.takes_value(true))
		.subcommand(SubCommand::with_name("collator"))
			.arg(Arg::with_name("c")
				.short("c")
				.help("collator in debug mode"))
		.subcommand(SubCommand::with_name("validator"))
			.arg(Arg::with_name("v")
				.short("v")
				.help("validator in debug mode"))
		.get_matches();

	// Gets value for config if supplied by user, or default to "default.conf"
	let log_pattern = matches.value_of("log").unwrap_or("default.conf");
	init_logger(log_pattern);
	println!("Value for config log pattern: {}", log_pattern);

	// Handle subcommand information by requesting matches by name
	// Calling `.unwrap()` not safe since value required so use `if let`
	// to conditionally get value
	if let Some(_) = matches.subcommand_matches("collator") {
		if matches.is_present("c") {
			println!("Running as collator in debug mode...");
		} else {
			println!("Running as collator normally...");
		}
		return;
	}

	if let Some(_) = matches.subcommand_matches("validator") {
		if matches.is_present("v") {
			println!("Running as validator in debug mode...");
		} else {
			println!("Running as validator normally...");
		}
		return;
	}
}


fn init_logger(pattern: &str) {
	let mut builder = env_logger::Builder::new();
	// Disable info logging by default for some modules:
	builder.filter(Some("hyper"), log::LevelFilter::Warn);
	// Enable info for others.
	builder.filter(None, log::LevelFilter::Info);

	if let Ok(lvl) = std::env::var("RUST_LOG") {
		builder.parse(&lvl);
	}

	builder.parse(pattern);
	builder.default_format_timestamp(true);
	builder.init();

	error!("Example Error message");
	warn!("Example Error message");
	debug!("Example Debug message");

	if log_enabled!(log::Level::Info) {
		info!("Example Info message");
	}

	if log_enabled!(log::Level::Trace) {
		trace!("Example Trace message");
	}
}