/* learn_string */

fn main() {
    describe("Reference Example");
    reference_example();
    
    describe("Ownership Example");
    ownership_example();

    describe("Idiomatic Example");
    idiomatic_example();
}

fn reference_example() {
    let first_name = String::from("Udit");
    let name = &first_name;
    // how do I complete further processing on name?
    println!("Greetings, {}!", name);
}

fn ownership_example() {
    let first_name = String::from("Udit");
    //to_owned() seems temporary
    let name = first_name.to_owned() + " Subramanya";
    println!("Hello, {}!", name);       // modified name using temp ownership
    println!("Hello, {}!", first_name); // original string is still available
  }

fn idiomatic_example() {
    let mut first_name = String::from("Udit");
    first_name.push_str(" Subramanya");
    println!("Nice to meet you, {}!", first_name);
}

/*
    str/string arguments need to be pass by reference
    can see this in lib functions too
    note... (parameter : type)
    See more: http://xion.io/post/code/rust-string-args.html
*/
fn describe(s: &str) {
    println!("<< {} >>", s);
}