extern crate docopt;
extern crate flac;
extern crate rustc_serialize;

#[macro_use]
mod commands;

use docopt::Docopt;
use flac::{Stream, StreamProducer, StreamReader};
use flac::metadata::{self, Picture, SeekPoint, VorbisComment};

use std::env;
use std::io::{self, Write};
use std::fs::File;

use commands::{streaminfo, comments, seektable, picture};

const USAGE: &'static str = "
Usage: metadata <command> [<args>...]
       metadata [options]

Options:
  -h, --help  Show this message.
";

#[derive(Debug, RustcDecodable)]
struct Arguments {
  arg_command: Option<Command>,
  arg_args: Vec<String>,
}

#[derive(Debug, RustcDecodable)]
enum Command {
  StreamInfo,
  Comments,
  SeekTable,
  Picture,
}

fn main() {
  let args: Arguments = Docopt::new(USAGE)
    .and_then(|d| d.options_first(true).decode())
    .unwrap_or_else(|e| e.exit());

  match args.arg_command.unwrap() {
    Command::StreamInfo => {
      let sub_args: streaminfo::Arguments = Docopt::new(streaminfo::USAGE)
        .and_then(|d| d.argv(env::args()).decode())
        .unwrap_or_else(|e| e.exit());

      streaminfo::run(&sub_args)
    }
    Command::Comments   => {
      let sub_args: comments::Arguments = Docopt::new(comments::USAGE)
        .and_then(|d| d.argv(env::args()).decode())
        .unwrap_or_else(|e| e.exit());

      comments::run(&sub_args)
    }
    Command::SeekTable  => {
      let sub_args: seektable::Arguments = Docopt::new(seektable::USAGE)
        .and_then(|d| d.argv(env::args()).decode())
        .unwrap_or_else(|e| e.exit());

      seektable::run(&sub_args)
    }
    Command::Picture    => {
      let sub_args: picture::Arguments = Docopt::new(picture::USAGE)
        .and_then(|d| d.argv(env::args()).decode())
        .unwrap_or_else(|e| e.exit());

      picture::run(&sub_args)
    }
  }
}
