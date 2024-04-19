mod environments;
mod stack;
mod interpreter;
mod shell;

use shell::base_shell;

fn main() {
    base_shell()
}
