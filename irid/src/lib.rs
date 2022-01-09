//= USES ===========================================================================================

pub use irid_app::*;
pub use irid_assets::*;
pub use irid_renderer::*;

//= TYPE ALIASES ===================================================================================

pub type ApplicationBuilder<'a, L, P, V, I> = irid_app::ApplicationBuilder<'a, L, IridWindowBuilder, P, V, I>;
