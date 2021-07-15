use clap::{load_yaml, App};

mod colors;
mod img;
mod m2i;
mod map_data;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("map2image", Some(matches)) => m2i::map2img(matches),
        _ => unreachable!(),
    }
}
