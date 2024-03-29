//= USES =====================================================================

pub use irid_app::*;
pub use irid_assets::*;
pub use irid_render::*;

//= TYPE ALIASES =============================================================

pub type RendererConfig<'a, V> =
    irid_render::RendererConfig<'a, PerspectiveCamera, &'a str, &'a str, V, u16>;
