#[macro_use]
extern crate rocket;
use api::db::{connect, find, insert, Product};
use mongodb::bson::doc;
use rocket::serde::{json::Json, Deserialize};
use serde_json;

#[get("/products")]
fn products() -> String {
	let collection = connect().expect("could not connect to the db");
	let cursor = find(&collection, String::from("bed"), 26);
	let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
	let serialized = serde_json::to_string(&docs).unwrap();
	serialized
}

#[post("/products", data = "<input>")]
fn post_product(input: Json<Product2>) -> String {
	let collection = connect().expect("could not connect to the db");
	let result = insert(
		vec![Product {
			name: input.name.to_string(),
			price: input.price,
		}],
		&collection,
	);
	match result {
		Err(e) => return e.to_string(),
		Ok(_result) => return String::from("Done"),
	}
}

#[derive(FromForm, Deserialize, Debug)]
pub struct Product2 {
	name: String,
	price: u32,
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![products, post_product])
}
