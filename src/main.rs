#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Pasta{
    data: String
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/newPasta",data = "<pasta>")]
fn pasta(pasta: String) -> String {
    // Generate a random number to use as the file name/id
    let id = Uuid::new_v4();
    let id = id.to_string();
    let save_destination = "pastas/".to_owned() + &id + ".txt";

    // Save the pasta to a database in pastas/
    let file = File::create(save_destination);
    match file{
        Ok(mut file) => {
            file.write_all(pasta.as_bytes()).unwrap();
            id
        },
        Err(_) => return "Error saving pasta".to_string()
    }
}

#[get("/<id>")]
fn get_pasta(id: &str) -> String {
    // Get the pasta from the database
    let pasta_path = "pastas/".to_owned() + &id + ".txt";
    let file = File::open(pasta_path);
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return "Pasta not found".to_string()
    };
    let mut pasta = String::new();
    match file.read_to_string(&mut pasta) {
        Ok(_) => pasta,
        Err(_) => "Pasta not found".to_string()
    }
}

#[launch]
fn rocket() -> _ {
    // Set a limit of 64KiB for forms and 3MiB for JSON.
    rocket::build().mount("/", routes![hello])
    .mount("/", routes![pasta])
    .mount("/", routes![get_pasta])
}