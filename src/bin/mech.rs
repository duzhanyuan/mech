// # Mech

/*
 Mech Server is a wrapper around the mech runtime. It provides interfaces for 
 controlling the runtime, sending it transactions, and responding to changes.
*/

// ## Prelude

extern crate core;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Sender};
use std::io;

extern crate clap;
use clap::{Arg, App};

extern crate term_painter;
use term_painter::ToStyle;
use term_painter::Color::*;

extern crate mech;
use mech::{ProgramRunner, RunLoop, RunLoopMessage};
use mech::ClientHandler;

extern crate mech_server;

// ## Server Entry

fn main() {

  let matches = App::new("Mech")
    .version("0.0.1")
    .author("Corey Montella")
    .about("The Mech REPL. Default values for options are in parentheses.")
    .arg(Arg::with_name("mech_file_paths")
      .help("The files and folders from which to load .mec files")
      .required(false)
      .multiple(true))
    .arg(Arg::with_name("serve")
      .short("s")
      .long("serve")
      .help("Starts a Mech HTTP and websocket server (false)"))
    .arg(Arg::with_name("port")
      .short("p")
      .long("port")
      .value_name("PORT")
      .help("Sets the port for the Mech server (3012)")
      .takes_value(true))
    .arg(Arg::with_name("http-port")
      .short("t")
      .long("http-port")
      .value_name("HTTPPORT")
      .help("Sets the port for the HTTP server (8081)")
      .takes_value(true))
    .arg(Arg::with_name("address")
      .short("a")
      .long("address")
      .value_name("ADDRESS")
      .help("Sets the address of the server (127.0.0.1)")
      .takes_value(true))
    .arg(Arg::with_name("persist")
      .short("r")
      .long("persist")
      .value_name("PERSIST")
      .help("The path for the file to load from and persist changes (current working directory)")
      .takes_value(true))
    .get_matches();

  let wport = matches.value_of("port").unwrap_or("3012");
  let hport = matches.value_of("http-port").unwrap_or("8081");
  let address = matches.value_of("address").unwrap_or("127.0.0.1");
  let serve = matches.is_present("serve");
  let http_address = format!("{}:{}",address,hport);
  let websocket_address = format!("{}:{}",address,wport);
  let mech_paths = matches.values_of("mech_file_paths").map_or(vec![], |files| files.collect());
  let persistence_path = matches.value_of("persistence").unwrap_or("");

  println!("\n {}",  BrightBlack.paint("╔════════════════╗"));
  println!(" {}      {}      {}", BrightBlack.paint("║"), BrightYellow.paint("MECH"), BrightBlack.paint("║"));
  println!(" {}\n",  BrightBlack.paint("╚════════════════╝"));
  if serve {
    mech_server::http_server(http_address);
    mech_server::websocket_server(websocket_address, mech_paths, persistence_path);
  } else {
    let mut mech_core = mech::Core::new(100000,100);
    'REPL: loop {      
      // If we're not serving, go into a REPL
      print!("{}", Yellow.paint("~> "));
      let mut input = String::new();

      io::stdin().read_line(&mut input).unwrap();

      // Handle built in commands
      match input.trim() {
        "help" => {
          println!("Available commands are: help, quit, core, runtime");
        },
        "quit" | "exit" => {
          break 'REPL;
        },
        "core" => {
          println!("{:?}", mech_core);
        }
        "runtime" => {
          println!("{:?}", mech_core.runtime);
        }
        _ => {
          let mut compiler = mech::Compiler::new();
          compiler.compile_string(input.trim().to_string());
          println!("Compiled {} blocks.", compiler.blocks.len());
          mech_core.register_blocks(compiler.blocks);
          mech_core.step();
        }
      }
    }
  }
}