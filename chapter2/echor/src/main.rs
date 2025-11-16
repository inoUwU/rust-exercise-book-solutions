use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Fract")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1), // 1つ以上指定
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    /* TODO: Updateで戻り値が変わったかも
    本家のリポジトリを確認してみる */
    let text = matches.value_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    // println!("{:#?}", matches)
    let text_vec = [text.into_owned()];

    println!(
        "{}{}",
        text_vec.join(" "),
        if omit_newline { "" } else { "\n" }
    );
}
