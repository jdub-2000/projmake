use std::env;
use std::fs;
use std::process::exit;

use serde_derive::Deserialize;
use toml;


struct Data {
    config: Config,
}

struct Config {
    dorp: String,
    glorp: String,
}


fn mk_proj_dir(project_name: &String, sub_dir: &str){

    match fs::create_dir_all(project_name.clone() + sub_dir) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    };

}


fn main() {

    let args: Vec<String> = env::args().collect();

    println!("Building directories for project named {:?} . . . ", args[1]);
    
    // declare/initialize variables
    let project_name = String::from(args[1].clone());
    let proj_dirs = ["/CODE", "/DAT", "/DOC", "/PROD", "/MODELS"];
    let mut idx = 0;

    
    while idx < proj_dirs.len() {

        mk_proj_dir(&project_name, proj_dirs[idx]);
        idx += 1;
    }



    println!(". . . Finished.");
}
