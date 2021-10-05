mod print;
mod vars;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod funcs;
mod ptr_ref;
mod structs;
mod string;
mod tuples;
mod enums;
mod cli;

fn main() {
    print::run();
    vars::run();
    arrays::run();
    vectors::run();
    loops::run();
    conditionals::run();
    funcs::run();
    ptr_ref::run();
    structs::run();
    string::run();
    tuples::run();
    enums::run();
    cli::run();
}
