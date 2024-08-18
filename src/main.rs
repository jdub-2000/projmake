use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    // first arg is always the path that called the program to execute
    // println!("My path is {}.", args[0]);

    println!("Setting up project directories for project named {:?} . . . ", args[1]);
    let project_name = String::from(args[1].clone());

    // Create a directory, returns `io::Result<()>`
    // create root of proj folder
    match fs::create_dir(project_name.clone()) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    };

    // subdirs need this different method cuz reasons
    match fs::create_dir_all(project_name.clone() + "/CODE") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    };

    match fs::create_dir_all(project_name.clone() + "/DAT") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    };

    match fs::create_dir_all(project_name.clone() + "/DOC") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    };

    match fs::create_dir_all(project_name.clone() + "/PROD") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    };

    println!(". . . Finished.");
}
