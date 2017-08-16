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
	fee: String,
	sendingaddress: String,
	#[serde(default="default_string")]
	referenceaddress: String,
	ismine: bool,
	version: u8,
	type_int: u8,
	propertyid: u8,
	divisible: bool,
	amount: String,
	valid: bool,
	blockhash: String,
	blocktime: u32,
	positioninblock: u32,
	block: u32,
	confirmations: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct OmniInfo {
	omnicoreversion_int: u32,
	omnicoreversion: String,
	mastercoreversion: String,
	bitcoincoreversion: String,
	block: u32,
	blocktime: u32,
	blocktransactions: u32,
	totaltrades: u32,
	alerts: Vec<String>
}

fn default_string() -> String {
	"reference".to_string()
}
#[derive(Debug, Deserialize, Serialize)]
struct RawTxn {
	rawtx: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct unconfirmedTransactions {
	txid: String,
	fee: String,
	sendingaddress: String,
	referenceaddress: String,
	ismine: bool,
	version: u32,
	type_int: u32,
	#[serde(rename = "type")]
	type_type: String,
	propertyid: u32,
	divisible: bool,
	amount: String,
	confirmations: u32
}

#[derive(Debug, Deserialize, Serialize)]
struct Payload {
	amount: String,
}

fn main() {
	let mut router = Router::new();

	router.post("/balance", move |r: &mut Request| get_balance(r), "get_balance");
	router.post("/transactions", move |r: &mut Request| get_transactions(r), "get_transactions");
	router.post("/broadcast", move |r: &mut Request| broadcast(r), "broadcast");
	router.post("/unconfirmed", move |r: &mut Request| unconfirmed(r), "unconfirmed");
	router.get("/getfee", move |r: &mut Request| getfee(r), "getfee");
	router.post("/getpayload", move |r: &mut Request| getpayload(r), "getpayload");
	router.get("/blockheight", get_blockheight, "get_blockheight");

	//route for get balance, accepts a public key, and a property identifier

	fn get_balance(req: &mut Request) -> IronResult<Response> {

		//todo get rid of unwraps
		let mut payload = String::new();
		req.body.read_to_string(&mut payload).unwrap();
		let address: Address = serde_json::from_str(&payload).unwrap();
		let balance = Command::new("omnicore-cli").arg("omni_getbalance").arg(address.address).arg("56").output().expect("failed");
		

		let output = balance.stdout;
		let mut response = Response::with((status::Ok, output));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));	
		response.set_mut(Header(headers::AccessControlAllowMethods(vec![Method::Post])));					
		Ok(response)
	}

	fn getpayload(req: &mut Request) -> IronResult<Response> {

		//todo get rid of unwraps
		let mut payload = String::new();
		req.body.read_to_string(&mut payload).unwrap();
		let payload: Payload = serde_json::from_str(&payload).unwrap();
		let payload = Command::new("omnicore-cli").arg("omni_createpayload_simplesend").arg("56").arg(payload.amount).output().expect("failed");
		

		let output = payload.stdout;
		let mut response = Response::with((status::Ok, output));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));	
		response.set_mut(Header(headers::AccessControlAllowMethods(vec![Method::Post])));					
		Ok(response)
	}

	fn getfee(req: &mut Request) -> IronResult<Response> {

		let fee = Command::new("omnicore-cli").arg("estimatefee").arg("6").output().expect("failed");
		
		 let mut s = String::from_utf8(fee.stdout).unwrap();
    		s.pop();
		let mut response = Response::with((status::Ok, s));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));	
		response.set_mut(Header(headers::AccessControlAllowMethods(vec![Method::Post])));					
		Ok(response)
	}


	fn unconfirmed(req: &mut Request) -> IronResult<Response> {
		//todo get rid of unwraps
		let mut payload = String::new();
		req.body.read_to_string(&mut payload).unwrap();
		let address: Address = serde_json::from_str(&payload).unwrap();
		let output = Command::new("omnicore-cli").arg("omni_listpendingtransactions").arg(&address.address).output().expect("failed");
		let output = String::from_utf8(output.stdout).unwrap();
		let output: Vec<unconfirmedTransactions> = serde_json::from_str(&output).unwrap();
		let mut pending_balance = 0;
		for txn in output {
			if address.address == txn.sendingaddress {
				pending_balance -= txn.amount.parse::<i32>().unwrap();
			} else {
				pending_balance += txn.amount.parse::<i32>().unwrap();
			}
		}


		let mut response = Response::with((status::Ok, pending_balance.to_string()));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));	
		response.set_mut(Header(headers::AccessControlAllowMethods(vec![Method::Post])));					
		Ok(response)
	}

	fn broadcast(req: &mut Request) -> IronResult<Response> {
		//todo get rid of unwraps
		let mut payload = String::new();
		req.body.read_to_string(&mut payload).unwrap();
		let txn: RawTxn = serde_json::from_str(&payload).unwrap();
		let output = Command::new("omnicore-cli").arg("sendrawtransaction").arg(txn.rawtx).output().expect("failed");
		

		let output = output.stdout;
		let mut response = Response::with((status::Ok, output));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));	
		response.set_mut(Header(headers::AccessControlAllowMethods(vec![Method::Post])));					
		Ok(response)
	}

	fn get_transactions(req: &mut Request) -> IronResult<Response> {
		println!("{:?}", req);
		//todo remove unwraps
		let mut payload = String::new();
		req.body.read_to_string(&mut payload).unwrap();
		println!("got through read to string");
		let txid: TxId = serde_json::from_str(&payload).unwrap();
		println!("didnt get through txid");


		let transaction = Command::new("omnicore-cli").arg("omni_gettransaction").arg(txid.txid).output().expect("failed");

		println!("got the command in");

		let output = transaction.stdout;

		let string_output = String::from_utf8_lossy(&output);

		let safex_tx: OmniTransaction = serde_json::from_str(&string_output).unwrap();


		if safex_tx.propertyid == 56 {

			let mut response = Response::with((status::Ok, serde_json::to_string(&safex_tx).unwrap()));
			response.set_mut(Header(headers::AccessControlAllowOrigin::Any));	
			response.set_mut(Header(headers::AccessControlAllowMethods(vec![Method::Post])));

			Ok(response)
		} else {

			Ok(Response::with((status::Ok)))
		}

	}

	fn get_blockheight(_: &mut Request) -> IronResult<Response> {
		let block_info = Command::new("omnicore-cli").arg("omni_getinfo").output().expect("failed");

		let output = block_info.stdout;

		let string_output = String::from_utf8_lossy(&output);
		let block_info: OmniInfo = serde_json::from_str(&string_output).unwrap();

		let mut response = Response::with((status::Ok, serde_json::to_string(&block_info).unwrap()));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));	
		response.set_mut(Header(headers::AccessControlAllowMethods(vec![Method::Post])));	

		Ok(response)
	}
	
	Iron::new(router).http("localhost:3001").unwrap();
}
