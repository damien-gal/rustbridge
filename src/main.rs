use clap::Parser;

extern crate unicode_width;

use unicode_width::UnicodeWidthStr;

///A little CLI that outputs a message.
// Those are doc comments. Most documentation is written this way.

#[derive(Debug, Parser)]
//for every struct, it is useful to derive Debug,
//and allows to use dbg!
struct Arguments {
    /// The thing to say.
    message: String,
}

// cargo run -- hello: passes the argument "hello" to our CLI,
// and not as a cargo argument
// autre exemple: cargo run -- --help
fn main() {
    let args = Arguments::parse();
    let s = args.message;
    let l = UnicodeWidthStr::width(s.as_str());
    println!("{}", "-".repeat(l+4));
    println!("> {} <", s);
    println!("{}", "-".repeat(l+4));
}
