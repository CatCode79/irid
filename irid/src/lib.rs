//= USES ===========================================================================================

pub use irid_app::*;
pub use irid_assets::*;
pub use irid_renderer::*;

//= TYPE ALIASES ===================================================================================

pub type ApplicationBuilder<'a, L> = irid_app::ApplicationBuilder<'a, L, IridWindowConfig>;
