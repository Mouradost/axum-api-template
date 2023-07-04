use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_appender::rolling;

#[allow(unused)]
#[derive(Clone, clap::ValueEnum)]
pub enum LoggerOutput {
    File,
    Stdout,
}

pub fn setup_logger(
    logger_output: LoggerOutput,
    level: Level,
) -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    let subscribe_builder = tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .with_file(false)
        .with_thread_ids(false)
        .pretty(); // .compact()

    match logger_output {
        LoggerOutput::File => {
            let log_file = rolling::daily("./logs", "daily");
            subscribe_builder
                .with_writer(log_file)
                .with_ansi(false)
                .init();
        }
        LoggerOutput::Stdout => {
            subscribe_builder
                .with_ansi(true)
                .init();
        }
    }
    TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(level))
        .on_response(trace::DefaultOnResponse::new().level(level))
}
