use::log::{debug, warn, error, info};
use::env_logger::{Builder, Env};
use::std::io::Write;

///
pub fn init_logger(){
    let mut builder = Builder::from_env(Env::default().default_filter_or("info"));

    builder.format(|buf, record| {
        writeln!(buf,
            "[{} {}] - {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        )
    }).init();
}
