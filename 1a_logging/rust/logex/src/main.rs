use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

mod foo {
    mod bar {
        pub fn run() {
            log::warn!("warn");
            log::info!("info");
            log::debug!("debug");
        }
    }

    pub fn run() {
        log::warn!("warn");
        log::info!("info");
        log::debug!("debug");
        bar::run();
    }
}

fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("-"),
                record.args()
            )
        })
        // run with LOGEX_LOG="warn,logex::foo=info,logex::foo::bar=debug"
        .parse_env("LOGEX_LOG")
        // filter_level wins over environment
        .filter_level(LevelFilter::Info)
        .init();

    log::warn!("warn");
    log::error!("error");
    log::info!("info");
    log::debug!("debug");
    foo::run();
}
