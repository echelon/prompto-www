// Copyright (c) 2017 Brandon Thomas <bt@brand.io, echelon@gmail.com>

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

extern crate iron;
extern crate mount;
extern crate multipart;
extern crate prompto;
extern crate staticfile;

//use clap::{App, Arg, ArgMatches};
use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use mount::Mount;
use multipart::server::{Multipart, Entries, SaveResult};
use staticfile::Static;
use std::fs::File;
use std::path::Path;
use std::io::Read;

fn main() {
  // Parse command line args.
  /*let matches = App::new("trumpet")
      .arg(Arg::with_name("PORT")
           .short("p")
           .long("port")
           .help("Sets the port the server listens on.")
           .takes_value(true)
           .required(false))
      .get_matches();

  let port = get_port(&matches, 9000);*/

  let port = 9090;
  start_server(port);
}

/*fn get_port(matches: &ArgMatches, default_port: u16) -> u16 {
  match matches.value_of("PORT") {
    None => default_port,
    Some(port) => {
      match port.parse::<u16>() {
        Err(_) => default_port,
        Ok(p) => p,
      }
    },
  }
}*/

fn start_server(port: u16) {
  let mut mount = Mount::new();
  mount.mount("/", Static::new(Path::new("www/index.html")));
  mount.mount("/upload", upload_handler);

  info!("Starting server on port {}...", port);
  Iron::new(mount).http(("0.0.0.0", port)).unwrap();
}

/// Processes a request and returns response or an occured error.
fn upload_handler(request: &mut Request) -> IronResult<Response> {
  // Getting a multipart reader wrapper
  match Multipart::from_request(request) {
    Ok(mut multipart) => {
      // Fetching all data and processing it.
      // save_all() reads the request fully, parsing all fields and saving all files
      // in a new temporary directory under the OS temporary directory.
      match multipart.save_all() {
        SaveResult::Full(entries) => {
          process_entries(entries)
        },
        SaveResult::Partial(entries, error) => {
          try!(process_entries(entries));
          Err(IronError::new(error, status::InternalServerError))
        }
        SaveResult::Error(error) => Err(IronError::new(error, status::InternalServerError)),
      }
    }
    Err(_) => {
      Ok(Response::with((status::BadRequest, "The request is not multipart")))
    }
  }
}

/// Processes saved entries from multipart request.
/// Returns an OK response or an error.
fn process_entries(entries: Entries) -> IronResult<Response> {

  for (name, field) in entries.fields {
    println!(r#"Field "{}": "{}""#, name, field);
  }

  for (name, saved_file) in entries.files {
    println!("Name: {:?}", name);
    println!("File: {:?}", saved_file);
    info!("Saved file: {:?}", saved_file.path);

    let mut bytes = Vec::new();
    let filename = saved_file.path.to_str().unwrap();
    let mut file = File::open(filename).unwrap();

    let _r = file.read_to_end(&mut bytes).unwrap();

    println!("Selfieing...");
    //let image = prompto::selfie_from_file(filename).unwrap();
    let image = prompto::selfie_from_memory(&bytes).unwrap();

    println!("Encoding...");
    let mut out_bytes = Vec::new();
    let _r = image.save(&mut out_bytes, prompto::ImageFormat::JPEG).unwrap();


    let mime_type = "image/jpeg".parse::<Mime>().unwrap();

    println!("Mime type is: {}", mime_type);

    let mut response = Response::with((mime_type, status::Ok, out_bytes));

    //response.headers.set(ETag(entity_tag));

    return Ok(response)
  }


  /*for (name, savedfile) in entries.files {
    let filename = match savedfile.filename {
      Some(s) => s,
      None => "None".into(),
    };

    info!("Saved file: {:?}", savedfile.path);

    let mut file = match File::open(savedfile.path) {
      Ok(file) => file,
      Err(error) => {
        return Err(IronError::new(error,
          (status::InternalServerError,
            "Server couldn't save file")))
      }
    };

    //prompto::selfie_from_memory();


    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
      return Err(IronError::new(error, (status::BadRequest, "The file was not a text")));
    }

    println!(r#"Field "{}" is file "{}":"#, name, filename);
    println!("{}", contents);
  }*/
  Ok(Response::with((status::Ok, "Multipart data is processed")))
}
