mod utils;

use clap::Parser;
use dialoguer::{FuzzySelect, Input, MultiSelect};
use num_bigint::BigInt;

///引数なし：自然数の入力後、動作をセレクト
/// --mersenne-search：メルセンヌ素数の探索をする
/// --to-perfect：メルセンヌ素数のpをもとに完全数に変換する。
#[derive(Parser)]
#[command(name = "Prime1")]
#[command(about = "自然数の解析を行うプログラム", long_about = None)]
struct Args {
    #[arg(short, long)]
    mersenne_search: bool,

    #[arg(short, long)]
    to_perfect: bool,
}

fn main() {
    let args = Args::parse();

    if args.mersenne_search {
        utils::prime::mersenne_search();
    } else if args.to_perfect {
        let options = utils::file::find_txt_files(".");
        if options.len() != 0 {
            let filename = FuzzySelect::new()
                .with_prompt("ファイルを選択してください")
                .items(&options)
                .interact()
                .unwrap();

            let filename = &options[filename];
            utils::file::file_convert(filename);
        } else {
            println!("ファイルが見つかりませんでした。");
        }
    } else {
        no_arguments();
    }
}

fn no_arguments() {
    let input = Input::<String>::new()
        .with_prompt("自然数を入力してください")
        .interact()
        .unwrap();
    let number: BigInt = input.trim().parse().unwrap();

    let options = &[
        "約数",
        "素数判定",
        "メルセンヌ素数判定",
        "完全数判定",
        "リュカ–レーマー・テスト",
    ];
    let selections = MultiSelect::new()
        .with_prompt("何を実行しますか？")
        .items(options)
        .interact()
        .unwrap();

    if selections.contains(&0) {
        let divisor = utils::prime::find_divisor(&number);
        println!("約数: {:?}", divisor);
    }
    if selections.contains(&1) {
        if utils::prime::miller_rabin(&number, 20) {
            println!("{} はたぶん素数です。", number);
            if utils::prime::is_prime(&number) {
                println!("{} は素数です。", number);
            } else {
                println!("{} は素数ではありません。", number);
            }
        } else {
            println!("{} は合成数です。", number);
        }
    }
    if selections.contains(&2) {
        if utils::prime::mersenne_primes_checker(&number) {
            println!("メルセンヌ素数です。");
        } else {
            println!("メルセンヌ素数ではありません。");
        }
    }
    if selections.contains(&3) {
        let divisor = utils::prime::find_divisor(&number);
        if utils::prime::perfect_number_checker(&number, &divisor) {
            println!("完全数です。");
        } else {
            println!("完全数ではありません。");
        }
    }
    if selections.contains(&4) {
        let num: usize = input.trim().parse().unwrap();
        if utils::prime::lucas_lehmer_test(num) {
            println!("M{} はメルセンヌ素数です。", num);
        } else {
            println!("M{} はメルセンヌ素数ではありません。", num);
        }
    }
}
