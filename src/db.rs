use mongodb::{
	bson::doc,
	error::Error,
	sync::{Client, Collection, Cursor},
};
use serde::{Deserialize, Serialize};

pub fn connect() -> Result<Collection<Product>, Error> {
	use std::env::args;
	let args: Vec<String> = args().collect();
	let args = &args[1..args.len()];
	let database: &String = &args[0];
	let password: &String = &args[1];
	let db_url: String = format!(
		"mongodb+srv://surphury:{}@market.pbh6p.mongodb.net/{}?retryWrites=true&w=majority",
		password, database
	);
	let client = Client::with_uri_str(db_url).expect("could not connect to the db");
	let database = client.database("market");
	let collection = database.collection::<Product>("products");
	return Ok(collection);
}

pub fn insert(
	docs: Vec<Product>,
	collection: &Collection<Product>,
) -> Result<mongodb::results::InsertManyResult, Error> {
	collection.insert_many(docs, None)
}

pub fn find(collection: &Collection<Product>, parametre: String, value: u32) -> Cursor<Product> {
	let cursor = collection
		.find(doc! { parametre: value }, None)
		.expect("Error on connection");
	return cursor;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
	pub name: String,
	pub price: u32,
}
