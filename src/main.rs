extern crate hyper;
extern crate html5ever;
extern crate tendril;

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

use std::io::Read;
use hyper::client::Client;

use tendril::*;
use tendril::fmt::{UTF8};

use html5ever::tokenizer::{TokenSink, Token, TokenizerOpts, ParseError};
use html5ever::tokenizer::{TagToken, StartTag, Tag};
use html5ever::driver::{tokenize_to, one_input};

struct LinkFinder{
    links: Vec<String>
}

impl TokenSink for LinkFinder {
    fn process_token(&mut self, token: Token) {
        match token {
            TagToken(tag @ Tag{kind: StartTag, ..}) => {
                if tag.name.as_slice() == "a" {
                    for attr in tag.attrs {
                        if attr.name.local.as_slice() == "href" {
                            self.links.push(String::from(attr.value));
                        }
                    }
                }
            }
            _ => ()
        }
    }
}

fn main() {
    let client = Client::new();

    let res = client.get("http://rust-lang.org").send();

    let mut response = match res {
        Ok(x) => x,
        Err(err) => panic!("{:?}", err)
    };

    let mut body = String::new();

    response.read_to_string(&mut body);

    let mut sink = LinkFinder{links: vec![]};

    sink = tokenize_to(sink, one_input(Tendril::from(body)), Default::default());

    let l = sink.links;

    let data = Arc::new(Mutex::new(l));

    let mut handlers = vec![];

    for i in 0..5 {
        let data = data.clone();
        handlers.push(thread::spawn(move || {
            loop {
                let link = {
                    data.lock().unwrap().pop()
                };
                println!("Starting to download this at thread {}", i); TODO actually download
                thread::sleep_ms(2000);
                match link {
                    Some(d) => println!("Finished downloading {} at thread {}", d, i),
                    None => break
                }
            }
        }));
    }

    for handler in handlers {
        handler.join();
    }

}
