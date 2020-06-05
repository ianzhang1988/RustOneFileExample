use reqwest::blocking::Response;
// use serde_json;

fn print_respond(mut res: Response) -> Result<(), Box<dyn std::error::Error>> {
    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());
    // copy the response body directly to stdout
    res.copy_to(&mut std::io::stdout())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // std::env::set_var("RUST_LOG", "reqwest=trace");
    env_logger::init();

    println!("GET http://127.0.0.1:8000");

    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .build()?;

    let res = client.get("http://localhost:8000/").send()?;
    print_respond(res)?;

    let res = client.get("http://127.0.0.1:8000/ian").send()?;
    print_respond(res)?;

    // let res = reqwest::blocking::get("http://localhost:8000/")?;
    // print_respond(res)?;

    // let res = reqwest::blocking::get("http://127.0.0.1:8000/ian")?;
    // print_respond(res)?;

    let res = client.post("http://localhost:8000/add")
        .body(r#"{"id": "1", "name": "ian.zhang", "age": 11, "phone": [ "12345", "67890" ], "address":{"home":"xxxx",  "work":"yyyy" }}"#)
        .send()?;
    print_respond(res)?;

    let res = client.post("http://localhost:8000/add")
        .body(r#"{"id": "2", "name": "ian", "age": 11, "phone": [ "12345", "67890" ], "address":{"home":"mood",  "work":"yyyy" }}"#)
        .send()?;
    print_respond(res)?;

    let res = client.post("http://localhost:8000/query")
        .body(r#"{"id": "2", "name": "ian"}"#)
        .send()?;
    print_respond(res)?;

    println!("\n\nDone.");
    Ok(())
}