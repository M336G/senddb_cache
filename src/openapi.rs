use utoipa::OpenApi;

use crate::{endpoints};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "SendDB Cache API",
        version = env!("CARGO_PKG_VERSION"),
        description = "A caching proxy for SendDB focusing on efficiently checking if levels have been sent or not ",
        contact(
            name = "M336",
            url = "https://m336.is-a.dev/"
        )
    ),
    servers(
        (url = "https://sdbc.m336.dev/", description = "SendDB Cache API")
    ),
    paths(
        endpoints::health::health_check,
        endpoints::stats::get_stats,
        endpoints::level::check_level
    ),
    tags(
        (name = "Health", description = "Health check endpoint(s)"),
        (name = "Statistics", description = "Statistics/information endpoint(s)"),
        (name = "Levels", description = "Level checks endpoint(s)")
    )
)]
pub struct ApiDoc;