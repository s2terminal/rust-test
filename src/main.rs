use structopt::StructOpt;

// use ferris_says::say;
// use std::io::{stdout, BufWriter};

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    // 1: expect
    // let content = std::fs::read_to_string(&args.path).expect("ファイル無いよ");

    // 2: match Err
    // let result = std::fs::read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => { content }
    //     Err(error) => { panic!("ファイル無いよ: {}", error); }
    // };

    // 3: unwrap
    let content = std::fs::read_to_string(&args.path).unwrap();

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // let pattern = std::env::args().nth(1).expect("なにもないよ");
    // let path = std::env::args().nth(2).expect("ぜんぜんなにもないよ");
    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    // let stdout  = stdout();
    // let message = String::from("こんにちは!");
    // let width   = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();
}
