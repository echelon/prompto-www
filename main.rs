// Copyright (c) 2015-2016 Brandon Thomas <bt@brand.io>

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

extern crate chrono;
extern crate clap;
extern crate crypto;
extern crate hound;
extern crate iron;
extern crate mount;
extern crate regex;
extern crate resolve;
extern crate router;
extern crate rustc_serialize;
extern crate staticfile;
extern crate time;
extern crate toml;
extern crate urlencoded;

pub mod config;
pub mod error;
pub mod handlers;
pub mod lang;
pub mod logger;
pub mod old_dictionary;
pub mod speaker;
pub mod synthesis;

use clap::{App, Arg, ArgMatches};
use config::Config;
use handlers::audio_synth_handler::AudioSynthHandler;
use handlers::error_filter::ErrorFilter;
use handlers::html_handler::HtmlHandler;
use handlers::vocab_list_handler::VocabListHandler;
use iron::prelude::*;
use lang::abbr::AbbreviationsMap;
use lang::arpabet::Arpabet;
use lang::dictionary::UniversalDictionary;
use lang::parser::Parser;
use lang::tokenizer::Tokenizer;
use logger::SimpleLogger;
use mount::Mount;
use old_dictionary::VocabularyLibrary;
use resolve::hostname;
use router::Router;
use staticfile::Static;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use synthesis::audiobank::Audiobank;
use synthesis::synthesizer::Synthesizer;

fn main() {
  let config = Config::read("./config.toml").unwrap();
  let logger = SimpleLogger::new(config.clone());
  logger.init().unwrap();

  // Parse command line args.
  let matches = App::new("trumpet")
      .arg(Arg::with_name("PORT")
           .short("p")
           .long("port")
           .help("Sets the port the server listens on.")
           .takes_value(true)
           .required(false))
      .get_matches();

  let port = get_port(&matches, 9000);

  VocabularyLibrary::read_from_directory(
      Path::new(&config.sound_path.clone().unwrap())).unwrap();

  get_hostname();

  let (parser, synthesizer) = create_parser_and_synthesizer(&config);

  start_server(&config, port, parser, synthesizer);
}

fn get_port(matches: &ArgMatches, default_port: u16) -> u16 {
  match matches.value_of("PORT") {
    None => default_port,
    Some(port) => {
      match port.parse::<u16>() {
        Err(_) => default_port,
        Ok(p) => p,
      }
    },
  }
}

fn start_server(config: &Config,
                port: u16,
                parser: Parser,
                synthesizer: Synthesizer) {
  let audio_path = &config.sound_path.clone().unwrap();
  let file_path = "./web";

  let async_synth = Arc::new(RwLock::new(synthesizer));

  // TODO: Cross-cutting filter installation
  let main_router = {
    let synth_handler = AudioSynthHandler::new(
      parser,
      async_synth.clone(),
      config.clone()
    );

    let mut chain = Chain::new(synth_handler);
    chain.link_after(ErrorFilter);

    let mut router = Router::new();

    // Json Endpoints
    router.get("/speak", chain);
    router.get("/words", VocabListHandler::new(audio_path));

    // Html Pages
    router.get("/", HtmlHandler::new(config.clone()));
    router.get("/old", HtmlHandler::new(config.clone()));

    router
  };

  let asset_router = {
    let handler = Static::new(Path::new(file_path));
    Chain::new(handler)
  };

  let mut mount = Mount::new();
  mount.mount("/assets", asset_router);
  mount.mount("/", main_router);

  info!("Starting server on port {}...", port);
  Iron::new(mount).http(("0.0.0.0", port)).unwrap();
}

fn get_hostname() {
  match hostname::get_hostname() {
    Ok(s) => { info!("Hostname: {}", s); },
    Err(_) => {},
  };
}

fn create_parser_and_synthesizer(config: &Config) -> (Parser, Synthesizer) {
  info!("Reading Arpabet Dictionary...");
  let arpabet_dictionary = Arpabet::load_from_file(
      &config.phoneme_dictionary_file.clone().unwrap()).unwrap();

  info!("Reading Extra Dictionary...");
  let extra_dictionary = Arpabet::load_from_file(
      &config.extra_dictionary_file.clone().unwrap()).unwrap();

  info!("Reading Square Dictionary...");
  let square_dictionary = Arpabet::load_from_file(
      &config.square_dictionary_file.clone().unwrap()).unwrap();

  let arpabet = arpabet_dictionary
      .combine(&extra_dictionary)
      .combine(&square_dictionary);

  let mut dictionary = UniversalDictionary::new();
  dictionary.set_arpabet_dictionary(arpabet.to_dictionary());

  let dictionary = Arc::new(dictionary);

  info!("Reading Abbreviations Map...");
  let abbreviations = Arc::new(AbbreviationsMap::load_from_file(
      &config.abbreviation_file.clone().unwrap()).unwrap());

  let tokenizer = Tokenizer::new(dictionary.clone(), abbreviations.clone());
  let parser = Parser::new(tokenizer, dictionary.clone(), abbreviations.clone());

  info!("Building Synthesizer...");
  let audiobank = Audiobank::new(&config.sound_path.clone().unwrap());
  let synthesizer = Synthesizer::new(arpabet, audiobank);

  (parser, synthesizer)
}

