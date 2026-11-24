mod auth;
mod cli;
mod gateway;
mod inbox;
mod job;
mod keys;
mod net;
mod security;
mod support;
mod wire;

fn watch_interrupt() {
    std::thread::spawn(|| {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(_) => return,
        };
        rt.block_on(async {
            if tokio::signal::ctrl_c().await.is_ok() {
                crate::job::stop::now();
            }
        });
    });
}

#[tokio::main]
async fn main() {
    watch_interrupt();
    cli::dispatch::run().await;
}
