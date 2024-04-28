use tracing_subscriber::prelude::*;

pub fn main() {
    let fmt_layer = tracing_subscriber::fmt::Layer::default();

    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::info!("Fmt Layer");
}
