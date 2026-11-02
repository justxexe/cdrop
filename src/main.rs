use anyhow::{Context, Result, anyhow};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let path = std::env::args().nth(1).context("укажите путь к файлу")?;
    let exp_hash = std::env::args().nth(2);

    let mut file = File::open(&path).with_context(|| format!("файл {path} не найден"))?;
    let size = file.metadata()?.len();

    let bar = ProgressBar::new(size);
    bar.set_style(ProgressStyle::with_template(
        "{bar:40} {bytes}/{total_bytes} ({bytes_per_sec})",
    )?);

    let mut hasher = blake3::Hasher::new();
    let mut buf = vec![0u8; 1024 * 1024]; // буфер 1 МБ, переиспользуется

    loop {
        let n = file.read(&mut buf)?; // &mut — даём read право писать в буфер
        if n == 0 {
            break;
        } // 0 байт = конец файла
        hasher.update(&buf[..n]); // срез: только реально прочитанное
        bar.inc(n as u64);
    }
    let hash = hasher.finalize();
    bar.finish();
    match exp_hash {
        None => println!("{}", hash),
        Some(expected) if hash.to_hex().as_str().eq_ignore_ascii_case(&expected) => {
            println!("OK, {hash}, {path}")
        }
        Some(_) => return Err(anyhow!("файл поврежден")),
    }
    Ok(())
}
