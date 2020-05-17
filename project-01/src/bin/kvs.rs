use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use std::process::exit;

fn main() {
    let set = SubCommand::with_name("set")
        .about("set value with key")
        .arg(
            Arg::with_name("key")
                .value_name("KEY")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("value")
                .value_name("VALUE")
                .required(true)
                .takes_value(true),
        );
    let get = SubCommand::with_name("get").about("get value by key").arg(
        Arg::with_name("key")
            .value_name("KEY")
            .required(true)
            .takes_value(true),
    );
    let rm = SubCommand::with_name("rm")
        .about("remove value by key")
        .arg(
            Arg::with_name("key")
                .value_name("KEY")
                .required(true)
                .takes_value(true),
        );
    let matches = App::new(crate_name!())
        .about(crate_description!())
        // use crate_version! to pull the version number
        .version(crate_version!())
        .author(crate_authors!())
        .subcommands(vec![set, get, rm])
        .get_matches();

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
