mod best_times;
mod best_times_error;
mod best_times_file;

pub use best_times::BestTimes;
pub use best_times_error::BestTimesError;
pub use best_times_file::{default_path, load_or_default, save};
