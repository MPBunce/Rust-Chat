#[macro_use] extern crate rocket;


#[get("/world")]
fn world() -> &'static str{
    "Hello world from chat app!"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![world])
}