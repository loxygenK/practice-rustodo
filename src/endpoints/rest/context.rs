use warp::{Filter, filters::BoxedFilter};

use crate::{repository::Repositories, helpers::Shared};

pub fn with_db(repos: Shared<Repositories>) -> BoxedFilter<(Shared<Repositories>,)> {
    warp::any().map(move || repos.clone()).boxed()
}
