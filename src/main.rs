#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod servicies;

use servicies::routes_service::*;

fn main() {
    println!("========= Nekomata Responce Service Start =========");

    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}
