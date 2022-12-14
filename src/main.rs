use clap::{App, Arg};
use std::fs::File;
use std::env;
use std::io::{self, BufReader, BufWriter,ErrorKind, Result, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024; // 16 KB

fn main() -> Result<()> {

    // for (k, v) in std::env::vars_os() {
    //     println!("{:?} {:?}", k, v)
    // }

    let name = env!("CARGO_PKG_NAME");
    let author = env!("CARGO_PKG_AUTHORS");
    let version = env!("CARGO_PKG_VERSION");
    let matches = App::new(name)
        .version(version)
        .author(author)
        .args(&[
            Arg::with_name("infile")
                .help("Read from a file instead of stdin."),
            Arg::with_name("outfile")
                .short('o')
                .long("outfile")
                .takes_value(true)
                .help("Write output to a file instead of stdout."),
            Arg::with_name("silent")
                .short('s')
                .long("silent"),
        ])
        .get_matches();

        
    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    let silent = if matches.is_present("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };
    dbg!(infile, outfile, silent);
    
    let mut reader:Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut writer:Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::open(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        
        total_bytes += num_read;
        

        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break
            }
            return Err(e);

            // eprintln!("Oh no, an error!: {}", e.to_string());
            // std::process::exit(1);
        }
    }

    if !silent {
        eprintln!("\r{}", total_bytes);
    }

    Ok(())
}

/*

$ dd if=/dev/urandom bs=1024 count=128 of=myfile
$ cargo build
$ cat myfile | target/debug/pipeviewer > myfile2
$ diff myfile myfile2

$ echo '123' | PV_SILENT=something cargo run

dbg!() ?????????debug

cargo run -- somefile --outfile file.out -s
 PV_SILENT=1 cargo run -- somefile --outfile file.out 
*/
