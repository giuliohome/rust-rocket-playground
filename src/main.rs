#[macro_use] extern crate rocket;

use rocket::fs::FileServer;

use std::fs::File;
use std::io::Read;
use std::io::BufReader;

//use chrono::NaiveDateTime;
use reqwest::Client;
use std::env;

use rocket::serde::{Serialize, Deserialize, json::Json};



#[derive(Deserialize,Debug)]
struct WeatherResponse {
    list: Vec<ForecastEntry>,
}

#[derive(Deserialize,Debug)]
struct ForecastEntry {
    // dt: i64,
    dt_txt: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize,Debug)]
struct Main {
    temp: f32,
}

#[derive(Deserialize,Debug)]
struct Weather {
    description: String,
}

#[get("/forecast/<city>")]
async fn forecast(city: &str) -> Json<Vec<String>> {
    let api_key = env::var("OPENWEATHER_API_KEY").expect("API key not set");
    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=metric",
        city, api_key
    );

    let client = Client::new();
    /*let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // Disable SSL validation (not for production)
        .build()
        .expect("Failed to build client");
    match reqwest::get(&url).await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                // Print and parse the response text
                println!("API response: {}", text);
                match serde_json::from_str::<WeatherResponse>(&text) {
                    Ok(parsed) => println!("Parsed JSON: {:?}", parsed),
                    Err(e) => println!("Error parsing JSON: {}", e),
                }
            }
            Err(e) => println!("Failed to read response text: {}", e),
        },
        Err(e) => println!("Failed to make request: {}", e),
    }*/
    if let Ok(resp) = client.get(&url).send().await {
        if let Ok(weather) = resp.json::<WeatherResponse>().await {
            let forecasts: Vec<String> = weather.list.iter().map(|entry| {
                // let forecast_time = NaiveDateTime::from_timestamp(entry.dt, 0);
                let forecast_time_str = entry.dt_txt.to_string();
                let description = entry.weather.get(0).map_or(
                    "No description available.".to_string(),
                    |w| w.description.clone(),
                );
                format!(
                    "{}: {}°C, {}",
                    forecast_time_str, entry.main.temp, description
                )
            }).collect();

            return Json(forecasts);
        }
    }

    Json(vec![format!("{} using url {}","Failed to retrieve forecast data.".to_string(), url)])
}

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
            .mount("/", routes![hello, get_json, data, forecast])
            .mount("/", FileServer::from("src/public"))
}
