extern crate env_logger;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

pub fn main() {
	let yaml = load_yaml!("cli.yml");
	let matches = clap::App::from_yaml(yaml).get_matches();

	// Gets value for config if supplied by user, or default to "default.conf"
	let log_pattern = matches.value_of("log").unwrap_or("default.conf");
	init_logger(log_pattern);
	info!("Value for config log pattern: {}", log_pattern);

	// Handle subcommand information by requesting matches by name
	// Calling `.unwrap()` not safe since value required so use `if let`
	// to conditionally get value
	if let Some(_) = matches.subcommand_matches("collator") {
		if matches.is_present("c") {
			info!("Starting collator in debug mode...");
		} else {
			info!("Starting collator normally...");
		}
		return;
	}

	if let Some(_) = matches.subcommand_matches("validator") {
		if matches.is_present("v") {
			info!("Starting validator in debug mode...");
		} else {
			info!("Starting validator normally...");
		}
		return;
	}

	println!("No command given.\n");
	let _ = clap::App::from_yaml(yaml).print_long_help();
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