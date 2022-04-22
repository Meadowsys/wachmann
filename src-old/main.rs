// figure out how to tell cargo that a main.rs is somewhere else?
// or is this the best method?

use twilight_bot_utils::prelude::*;
mod bot;
use bot::main as _main;

fn main() -> MainResult { _main() }
