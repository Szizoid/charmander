mod config;
mod history;
mod search;

fn main() {
    let path = std::path::PathBuf::from("config/default.toml");
    let cfg = config::load(&path).expect("Не удалось загрузить конфиг");
    println!("{:#?}", cfg);
}
