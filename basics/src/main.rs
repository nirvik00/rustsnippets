mod arrays;
mod cli;
mod conditionals;
mod enums;
mod func_json;
mod func_json2;
mod funcs;
mod linear_algebra;
mod loops;
mod print;
mod ptr_ref;
mod string;
mod structs;
mod tuples;
mod vars;
mod vectors;

pub mod update_global_arr;

/* pub struct Person {
    lname: String,
    fname: String,
    age: u8,
} */

fn main() {
    /*
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
    func_json::run();
    func_json2::run();

    let mut s = String::from("hello");
    let mut x = 10;
    update_global_arr::run(&mut s, &mut x);
    println!("{:?}", (s, x));

    let mut p = update_global_arr::Person {
        lname: String::from("saha"),
        fname: String::from("nirvik"),
        age: 37,
    };
    update_global_arr::per(&mut p);
    println!("{:?}", &p);

    let b = update_global_arr::Cube::new(1.23, 2.54, 1.98, "b1");
    println!("id = {:?}", b.id);
    println!("vol = {:?}", b.vol());
    println!("area = {:?}", b.ar());
    println!("{}", b.display());
    */

    linear_algebra::run();
}
