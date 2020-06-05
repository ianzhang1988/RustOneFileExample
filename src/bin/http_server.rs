use actix_web::{web, middleware, App, HttpRequest, HttpServer, Responder, HttpResponse, web::{Bytes, post, Query} };
use serde_json::{self, json, Value };
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Person {
    age: u8,
    id: String,
    name: String,
    phone: Vec<String>,
    address: HashMap<String, String>,
}

type MyData = HashMap<String, Person>;

async fn handler(bytes: Bytes, query: Query<Person>) -> impl Responder {
    let name = String::from_utf8(bytes.to_vec()).map_err(|_| HttpResponse::BadRequest().finish())?;
    // Result<String, HttpResponse> Hmm
    Ok::<String, HttpResponse>(format!("Hello, {}!\nYou are user #{} and are {} years old.\n", name, query.id, query.age))
}

/// test json decode post data and write some json data back as string
///
/// input
/// ``` json
/// {
///     "name": "xxx",
///     "id": "xxx"
/// }
/// ```
/// output
/// ``` json
/// {
///     "id": "xxx",
///     "name": "xxx",
///     "age": 11,
///     "phone": [
///         "12345",
///         "67890"
///     ]
///     "address":{
///         "home":"xxxx",
///         "work":"yyyy"
///     }
/// }
/// ```
async fn query_id(bytes: Bytes) -> impl Responder {
    let json_data = String::from_utf8(bytes.to_vec()).map_err(|_| HttpResponse::BadRequest().finish())?;

    let v: Value = serde_json::from_str(json_data.as_str()).map_err(|_| HttpResponse::BadRequest().finish())?;

    let id: String = v["id"].to_string();
    let name: String = v["name"].to_string();

    println!("id {} name {}", id, name);

    let person = json!({
        "id": format!("{}", id),
        "name": name.as_str(),
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ],
        "address":{
            "home":"xxxx",
            "work":"yyyy"
        }
    });


    Ok::<String, HttpResponse>(format!("JSON data, {}!\n", person.to_string()))
}

/// {"id": "1", "name": "ian", "age": 11, "phone": [ "12345", "67890" ], "address":{"home":"xxxx",  "work":"yyyy" }}
/// parse json as typed data structure, add data to "App data" which can be using in different handler

async fn add(bytes: Bytes, data: web::Data<Arc<Mutex<MyData>>>) -> impl Responder {

    let json_data = String::from_utf8(bytes.to_vec()).map_err(|_| HttpResponse::BadRequest().finish())?;
    let person : Person = serde_json::from_str(json_data.as_str()).map_err(|_| HttpResponse::BadRequest().finish())?;
    // println!("person data {:?}", &person);

    let name = person.id.clone();

    // get the data, using actix web::Data extractor
    let mut person_map = data.lock().unwrap();
    person_map.insert(person.id.clone(), person);

    Ok::<String, HttpResponse>(format!("added person, {}!\n", name))
}

async fn query(bytes: Bytes, data: web::Data<Arc<Mutex<MyData>>>) -> impl Responder {

    let json_data = String::from_utf8(bytes.to_vec()).map_err(|_| HttpResponse::BadRequest().finish())?;
    let v: Value = serde_json::from_str(json_data.as_str()).map_err(|_| HttpResponse::BadRequest().finish())?;
    let id: String = v["id"].as_str().ok_or(HttpResponse::BadRequest().finish())?.to_string();

    println!("id: {:?}", id);

    let person_map = data.lock().unwrap();

    println!("person_map: {:?}", person_map);

    let json: serde_json::Value;

    if let Some(person) = person_map.get(&id){

        json = serde_json::to_value(person).unwrap();

    }
    else {
        json = json!({"error":"cant find"})
    }

    Ok::<HttpResponse,HttpResponse>(HttpResponse::Ok()
            .content_type("application/json")
            .body(json))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    // not working, `data: web::Data<Arc<Mutex<MyData>>>` web::Data must added some where else
    // let data = web::Data::new(Arc::new(Mutex::new(MyData::new())));
    let data = Arc::new(Mutex::new(MyData::new()));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(data.clone())
            .route("/", web::get().to(greet))
            .route("/name", post().to(handler))
            .route("/query_test", post().to(query_id))
            .route("/add", post().to(add))
            .route("/query", post().to(query))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}