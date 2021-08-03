#[macro_use]
extern crate diesel;
#[macro_use]
extern crate anyhow;

pub mod price_coingecko;
pub mod keeper_extractor;
pub mod events_extractor;
pub mod events_reports;
pub mod forum_extractor;
pub mod schema;

use diesel::r2d2::{ConnectionManager,Pool};
use diesel::PgConnection;
pub type DbPool = std::sync::Arc<Pool<ConnectionManager<PgConnection>>>;
