use anyhow::Result;
use log::{debug, error, info, warn};
use regex::Regex;
use std::env;
fn main() {
    mylogger::init();
    if let Err(e) = run() {
        error!("{:?}", e);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        warn!("Usage: {} args", args[0]);
        warn!("\tセットコードを引数に入力（例：WOE）");
        return Ok(());
    }

    let set_code: &str = &args[1];
    let re = Regex::new(r"^[A-Z]{3}$")?;

    if !re.is_match(set_code) {
        warn!("セットコード：{:?} は不正", set_code);
        return Ok(());
    }

    info!("セットコード：{:?}", set_code);
    // 大文字3文字は確認

    // 拡張機能作成
    Ok(())
}
