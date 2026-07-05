// IKinder

#![allow(deprecated)]

use time::format_description::BorrowedFormatItem;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    crate::show_name(file!());

    use tracing_subscriber::fmt::format::FmtSpan;

    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_timer(make_timer())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE | FmtSpan::EXIT)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Starting up!");
    tracing::warn!("Are you sure?");
    tracing::error!("Something went wrong.");
    hello_world().await;

    Ok(())
}

#[tracing::instrument]
async fn hello_world() {
    println!("Hello, world!");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}

fn make_timer<'a>() -> tracing_subscriber::fmt::time::OffsetTime<Vec<BorrowedFormatItem<'a>>> {
    let offset = time::UtcOffset::from_hms(3, 0, 0).expect("Should get PST offset");
    let time_format = time::format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]",
    )
    .expect("Format string should be valid");
    tracing_subscriber::fmt::time::OffsetTime::new(offset, time_format)
}
