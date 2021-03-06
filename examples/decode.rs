extern crate serialize;
extern crate docopt;

use docopt::docopt;

#[deriving(Decodable, Show)]
struct Args {
    flag_speed: int,
    flag_drifting: bool,
    arg_name: Vec<String>,
    arg_x: Option<int>,
    arg_y: Option<int>,
}

fn main() {
    let args: Args = docopt("
Naval Fate.

Usage:
  naval_fate.py ship new <name>...
  naval_fate.py ship <name> move <x> <y> [--speed=<kn>]
  naval_fate.py ship shoot <x> <y>
  naval_fate.py mine (set|remove) <x> <y> [--moored | --drifting]
  naval_fate.py (-h | --help)
  naval_fate.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --speed=<kn>  Speed in knots [default: 10].
  --moored      Moored (anchored) mine.
  --drifting    Drifting mine.
").unwrap_or_else(|e| e.exit()).decode_must();
    println!("{}", args);

    println!("\nSome values:");
    println!("  Speed: {}", args.flag_speed);
    println!("  Drifting? {}", args.flag_drifting);
    println!("  Names: {}", args.arg_name);
}
