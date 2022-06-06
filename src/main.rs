use std::env;
use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;


// TODO:
// - Fb2 format
//  - Get file name path  
//  - Get book information
//  - Read book content


fn parse_file(filename: &str) {
    let mut reader = Reader::from_file(filename).expect("Error with file"); 
    reader.trim_text(true);

    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = str::from_utf8(e.name()).unwrap();
                println!("Found element {:?} - {:?}", name, e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>());
            },
            Ok(Event::Text(e)) => println!("Text found in this buffer {:?}", e.unescape_and_decode(&reader).unwrap()),
            Err(e) => panic!("We fucked at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            _ => (),
        }

        buf.clear();
    }
}

fn main() { 
    let args: Vec<String> = env::args().collect();

    // You have to specifie the file 
    if args.len() <= 1 {
        eprintln!("You have to specifi the file to read\n  -- what [file]");
        return;
    }

    let filename = &args[1];
    parse_file(filename);
}
