#[macro_use]
extern crate clap;
use clap::App;

use exitfailure::ExitFailure;
use failure::ResultExt;

mod build;
mod new;
mod serve;

fn main() -> Result<(), ExitFailure> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("new", Some(matches)) => {
            if matches.is_present("name") {
                new::new_site(matches.value_of("name").unwrap());
            }
        }
        ("build", Some(_matches)) => {
            let source_dir = matches.value_of("source");
            let destination_dir = matches.value_of("destination").unwrap();

            build::build_site(source_dir, destination_dir);
        }
        ("serve", Some(_matches)) => {
            let source_dir = matches.value_of("source").unwrap();
            let destination_dir = matches.value_of("destination").unwrap();

            serve::serve_site(source_dir, destination_dir);
        }
        _ => unreachable!()
    }

    Ok(())
}
