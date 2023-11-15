#[macro_use] extern crate rocket;
#[macro_use] extern crate magic_crypt;
use magic_crypt::MagicCryptTrait;
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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SecurePasta{
    id: String,
    password: String
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
            let file_write = file.write_all(pasta.as_bytes());
            match file_write{
                Ok(_) => id,
                Err(_) => return "Error saving pasta to file".to_string()
            }
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

// We do the same, but with encrypted pastas
#[post("/newPastaSecure",data = "<pasta>")]
fn secure_pasta(pasta: String) -> String {
    let id = Uuid::new_v4();
    let id = id.to_string();
    let save_destination = "pastas/".to_owned() + &id + ".txt";

    // Generate a random password
    let password = Uuid::new_v4();
    let password = password.to_string();


    // Encrypt the pasta with the password
    let encrypted_pasta = encrypt(pasta, password.clone());

    let return_pasta = SecurePasta{
        id: id.clone(),
        password: password
    };

    // Save the pasta to a database in pastas/
    let file = File::create(save_destination);
    match file{
        Ok(mut file) => {
            let file_write = file.write_all(encrypted_pasta.as_bytes());
            match file_write{
                Ok(_) => {
                    let pasta_object_to_json = serde_json::to_string(&return_pasta);
                    match pasta_object_to_json{
                        Ok(pasta_object_to_json) => pasta_object_to_json,
                        Err(_) => return "Error converting pasta object to JSON".to_string()
                    }
                },
                Err(_) => return "Error saving pasta to file".to_string()
            }
        },
        Err(_) => return "Error creating pasta file".to_string()
    }
}

#[get("/<id>/<password>")]
fn get_secure_pasta(id: &str, password: &str) -> String {
    // Get the pasta from the database
    let pasta_path = "pastas/".to_owned() + &id + ".txt";
    let file = File::open(pasta_path);
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return "Pasta not found".to_string()
    };
    let mut pasta = String::new();
    match file.read_to_string(&mut pasta) {
        Ok(_) => decrypt(pasta, password.to_string()),
        Err(_) => "Pasta not found".to_string()
    }
}

fn decrypt(encrypted_pasta: String, password: String) -> String{
    let mcrypt = new_magic_crypt!(password, 256);
    let unencrypted_pasta = mcrypt.decrypt_base64_to_string(encrypted_pasta);
    match unencrypted_pasta{
        Ok(unencrypted_pasta) => unencrypted_pasta,
        Err(_) => "Wrong password or other error decrypting pasta".to_string()
    }
}

fn encrypt(unencrypted_pasta: String, password: String) -> String{
    let mcrypt = new_magic_crypt!(password, 256);
    let encrypted_pasta = mcrypt.encrypt_str_to_base64(unencrypted_pasta);

    encrypted_pasta
}

#[launch]
fn rocket() -> _ {
    // Set a limit of 64KiB for forms and 3MiB for JSON.
    rocket::build().mount("/", routes![hello])
    .mount("/", routes![pasta])
    .mount("/", routes![get_pasta])
    .mount("/", routes![secure_pasta])
    .mount("/", routes![get_secure_pasta])
}