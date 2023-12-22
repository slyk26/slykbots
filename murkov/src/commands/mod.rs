mod stats;
mod music;
mod toggle;

pub use crate::commands::stats::Stats;
pub use crate::commands::music::Music;
pub use crate::commands::toggle::Toggle;
pub use shared::serenity_utils::SlashCommand;
