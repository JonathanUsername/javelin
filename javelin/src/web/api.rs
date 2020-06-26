use {
    warp::{
        Filter,
        Reply,
        Rejection,
        filters::BoxedFilter,
    },
    serde_json::json,
    thiserror::Error,
    crate::Shared,
};


#[derive(Error, Debug)]
pub enum Error {
    #[error("No such resource")]
    NoSuchResource,

    #[error("Stream could not be found")]
    StreamNotFound,
}


pub(crate) fn api(shared: Shared) -> BoxedFilter<(impl Reply,)> {
    active_streams(shared.clone())
        .or(stream_stats(shared.clone()))
        .or(server_info())
        .or_else(|err: Rejection| {
            if err.is_not_found() {
                Err(warp::reject::custom(Error::NoSuchResource))
            } else {
                Err(err)
            }
        })
        .boxed()
}

fn active_streams(shared: Shared) -> BoxedFilter<(impl Reply,)> {
    warp::path("active-streams")
        .map(move || {
            let streams = shared.streams.read();
            let active = streams.iter()
                .map(|(k, _)| k.clone())
                .collect::<Vec<String>>();

            let json = json!({
                "streams": active
            });

            warp::reply::json(&json)
        })
        .boxed()
}

fn stream_stats(shared: Shared) -> BoxedFilter<(impl Reply,)> {
    warp::path("stream-stats").and(warp::path::param())
        .and_then(move |app_name: String| {
            let streams = shared.streams.read();
            match streams.get(&app_name) {
                Some(stream) => {
                    let metadata = stream.metadata.clone()
                        .map(|m| json!({
                            "video": {
                                "codec": m.get::<String, _>("video.codec"),
                                "bitrate": m.get::<u32, _>("video.bitrate"),
                                "framerate": m.get::<u32, _>("video.frame_rate"),
                                "width": m.get::<u32, _>("video.width"),
                                "height": m.get::<u32, _>("video.height")
                            },
                            "audio": {
                                "codec": m.get::<String, _>("audio.codec"),
                                "bitrate": m.get::<u32, _>("audio.bitrate_kbps"),
                                "sample_rate": m.get::<u32, _>("audio.sample_rate"),
                                "channels": m.get::<u8, _>("audio.channels")
                            }
                        }));

                    let json = json!({
                        "app_name": app_name,
                        "start_time": stream.publish_start,
                        "metadata": metadata
                    });
                    Ok(warp::reply::json(&json))
                },
                None => {
                    Err(warp::reject::custom(Error::StreamNotFound))
                }
            }
        })
        .boxed()
}

fn server_info() -> BoxedFilter<(impl Reply,)> {
    warp::path("server-info")
        .map(|| {
            let json = json!({
                "version": env!("CARGO_PKG_VERSION"),
                "authors": env!("CARGO_PKG_AUTHORS").split_terminator(':').collect::<Vec<_>>(),
                "backend": env!("CARGO_PKG_NAME"),
            });

            Ok(warp::reply::json(&json))
        })
        .boxed()
}
