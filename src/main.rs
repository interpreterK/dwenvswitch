/*
dwenvswitch -e kde-plasma --no-wayland-session

doas support if sudo is not installed
--download-only option
support multiple
*/
use clap::{command, Arg};



fn main() {
	let dwenvswitch_command = command!()
		.about("Switch to a desktop environment or window manager with optional patches via single command.")
		.arg(
			Arg::new("Environment Name")
				.long("environment")
				.short('e')
				.alias("env")
				.aliases(["environments", "envs"])//future plan
				.help("Specify the name of which desktop or window manager environment(s) to install")
				.required(true)
		)
		.arg(
			Arg::new("Environment Options")
				.long("options")
				.short('o')
				.aliases(["option", "opts", "opt"])
				.help("Specify options for corresponding environment(s)")
		)
		.get_matches();

	println!("{}", dwenvswitch_command.get_one::<String>("Environment Name").unwrap_or(&"NO BAD".to_string()))
}