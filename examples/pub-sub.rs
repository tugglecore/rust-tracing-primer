fn main () {
    let subscriber = tracing_subscriber::FmtSubscriber::new();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::info!("Set up our subscriber");
}
