#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};

use std::fs::File;
use std::io::Read;
use std::io::BufReader;

use rocket::serde::{Serialize, Deserialize, json::Json};


#[get("/api/data")]
fn data() -> Json<Vec<MyData>> {

    // Initial vector of MyData
    /* let mut data: Vec<MyData> = vec![
        MyData { id: 1, value: "First".to_string() },
        MyData { id: 2, value: "Second".to_string() },
        MyData { id: 3, value: "Third".to_string() },
    ]; */


    //  the  operator `?` can only be used in a function that returns `Result` or `Option`
    //let file = File::open("src/data.json")?;
    // Try to open the file
    let file = match File::open("json/data.json") {
        Ok(f) => f,
        Err(_) => return Json(vec![]), // If there's an error, return an empty vector
    };
    // Read the JSON file
    let reader = BufReader::new(file);

    // Deserialize JSON into a Vec<MyData>
    let mut data: Vec<MyData> = serde_json::from_reader(reader).unwrap();

    // Update a value (modify where id == 2)
    if let Some(item) = data.iter_mut().find(|x| x.id == 2) {
        item.value = "Updated Second".to_string();
    }

    // Add a new object
    let new_data = MyData {
        id: 4,
        value: "Fourth".to_string(),
    };
    data.push(new_data);

    // Delete an object (remove where id == 1)
    data.retain(|x| x.id != 1);

    // Output the final vector as JSON
    let json_data = serde_json::to_string(&data).unwrap();
    println!("{}", json_data);

    Json(data)
}

#[derive(Serialize, Deserialize)]
struct MyData {
    id: i32,
    value: String,
} 


#[get("/json")]
fn get_json() -> &'static str { 
    let mut file = File::open("json/input.json").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let static_str: &'static str = Box::leak(contents.into_boxed_str()); 
    static_str
}

#[get("/hello")]
fn hello() -> &'static str {
        "Hello, world!"
}

#[launch]
fn rocket() -> _ {
        rocket::build()
            .mount("/", routes![hello, get_json, data])
            .mount("/", FileServer::from(relative!("src/public")))
}
