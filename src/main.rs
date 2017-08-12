extern crate iron;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


extern crate router;

use router::Router;
use iron::prelude::*;
use iron::{status, headers};
use iron::method::Method;
use iron::modifiers::Header;

use std::io::Read;
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
struct Address {
    address: String
}

#[derive(Debug, Deserialize, Serialize)]
struct TxId {
	txid: String
}

#[derive(Debug, Deserialize, Serialize)]
struct OmniTransaction {
	#[serde(rename = "type")]
	type_type: String,
	txid: String,
	fee: f64,
	sendingaddress: String,
	referenceaddress: String,
	ismine: bool,
	version: u8,
	type_int: u8,
	propertyid: u8,
	divisible: bool,
	amount: f64,
	valid: bool,
	blockhash: String,
	blocktime: u32,
	positionblock: u32,
	block: u32,
	confirmations: u32,
}

fn main() {
	let mut router = Router::new();

	router.post("/balance", move |r: &mut Request| get_balance(r), "get_balance");
	router.post("/transactions", move |r: &mut Request| get_transactions(r), "get_transactions");

	//route for get balance, accepts a public key, and a property identifier

	fn get_balance(req: &mut Request) -> IronResult<Response> {
		println!("{:?}", req);
		let mut payload = String::new();
		req.body.read_to_string(&mut payload).unwrap();
		let address: Address = serde_json::from_str(&payload).unwrap();
		let balance = Command::new("omnicore-cli").arg("omni_getbalance").arg(address.address).arg("56").output().expect("failed");
		println!("{:?}", balance);
		let output = balance.stdout;
		let mut response = Response::with((status::Ok, output));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));	
		response.set_mut(Header(headers::AccessControlAllowMethods(vec![Method::Post])));					
		Ok(response)
	}

	fn get_transactions(req: &mut Request) -> IronResult<Response> {
		let mut payload = String::new();
		req.body.read_to_string(&mut payload).unwrap();
		let txid: TxId = serde_json::from_str(&payload).unwrap();

		let transaction = Command::new("omnicore-cli").arg("omni_gettransactions").arg(txid.txid).output().expect("failed");

		Ok(Response::with((status::Ok)))
	}

	fn get_blockheight(_: &mut Request) -> IronResult<Response> {
		Command::new("omnicore-cli").arg("omni_getinfo").spawn().expect("failed");
		Ok(Response::with(status::Ok))
	}
	
	Iron::new(router).http("localhost:3001").unwrap();
}
