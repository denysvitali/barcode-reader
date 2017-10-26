extern crate serial;
extern crate serde_yaml;
extern crate serde_json;

// HTTP Client
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

mod barcodes;

use barcodes::barcode;
use barcodes::barcode::BarcodeType;

use std::io::{self, Write, Read, ErrorKind};
use futures::{Future, Stream};
use std::str;
use hyper::Client;
use tokio_core::reactor::Core;
use hyper::{Method, Request};
use hyper::header::{ContentLength, ContentType,Authorization};
use hyper_tls::HttpsConnector;

use serde_json::Value;

use std::error::Error;

use std::collections::BTreeMap;

use std::fs::File;

use std::time::Duration;
use serial::prelude::*;

static SETTINGS_FILE : &'static str = "settings.yml";

fn main() {
    let config: BTreeMap<String,String>;

    match load_config() {
        Ok(t) => {
            config = t;
            println!("Configuration loaded succesfully!");
        }
        Err(e) => {
            println!("Unable to load configuration! {:?}", e.kind());
            return;
        }
    }

    let mut openfood_apikey : &str;
    match config.get("openfood_apikey") {
        None => {
            println!("OpenFood API key not found!");
            return;
        },
        Some(t) => {
            openfood_apikey = t;
        }
    }

    println!("OpenFood API key: {}", openfood_apikey);

    //testRequest("7613331649478", openfood_apikey);

    // Open Serial
    let mut port = serial::open("/dev/ttyACM0").unwrap();
    loop {
        match interact(&mut port) {
            Ok(result) => {
                println!("Barcode from interact: {}", result);
                //testRequest(&result, openfood_apikey);
            },
            Err(e) => {
                if e.kind() == ErrorKind::TimedOut {
                    println!("Timed Out");
                }
                else{
                    println!("Error Kind: {:?}", e.kind());
                }
            }
        }
    }
}

fn load_config() -> Result<BTreeMap<String,String>,std::io::Error>{
    let mut path = String::from("./");
    path.push_str(SETTINGS_FILE);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let yaml : BTreeMap<String, String> = serde_yaml::from_str(&mut contents).unwrap();
    //println!("OpenFood API key: {}", yaml.get("openfood_apikey").unwrap());
    Ok(yaml)
}

fn testRequest(barcode: &str, key: &str) -> Result<(), Box<Error>>{
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    let mut json = r#"{
        "_source": {
            "includes": [
                "name_translations",
                "barcode"
            ]
        },
        "size": 20,
        "query": {
            "query_string": {
                "fields" : [
                    "barcode"
                ],
                "query" : "BARCODE_HERE"
            }
        }
    }
    "#;

    let json : String = json.replace("BARCODE_HERE", barcode);

    let uri = "https://www.openfood.ch/api/v3/products/_search".parse()?;
    let mut req : Request = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.headers_mut().set(Authorization(format!("Token token={}", key)));
    req.set_body(json);
    let post = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());
        res.body().concat2()
    });
    let posted = core.run(post).unwrap();
    let json_body : Value = serde_json::from_str(&String::from_utf8(posted.to_vec())?)?;
    let first_result : &Value = json_body["hits"]["hits"].get(0).unwrap();
    let names = first_result["_source"]["name_translations"].as_object().unwrap();
    println!("First result: {:?}", first_result);
    println!("Name en: {:?}", names.get("en"));
    println!("Name it: {:?}", names.get("it"));
    println!("Name de: {:?}", names.get("de"));
    println!("Name fr: {:?}", names.get("fr"));
    Ok(())
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<String>{
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(100000))?;

    let mut result: Vec<u8> = Vec::new();
    let mut buf = vec![1, 50];

    let mut carriage_return = false;
    while !carriage_return {
        port.read(&mut buf)?;
        for x in &buf{
            if *x == '\r' as u8 {
                carriage_return = true;
                break;
            }
            result.push(*x);
        }
    }
    let mut vec = result.clone();
    vec.clone_from_slice(&result);
    let barcode : String = String::from_utf8(vec).unwrap();
    let barcode_type = barcode::detect_type(&barcode);
    let mut s = String::new();
    println!("barcode type: {:?}", barcode_type);

    if barcode_type == BarcodeType::EAN13 || barcode_type == BarcodeType::Code128 {
        // Clean Barcode
        result.remove(0);
    }
    else {
        result.drain(0..2);
    }

    s = String::from_utf8(result).unwrap();

    println!("barcode: {:?}", s);

    Ok(s)
}
