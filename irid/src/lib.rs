//= USES ===========================================================================================

pub use irid_app::*;
pub use irid_assets::*;
pub use irid_renderer::*;

//= TYPE ALIASES ===================================================================================

pub type ApplicationConfig<'a, L, V> = irid_app::ApplicationConfig<'a, L, IridWindowConfig, V>;

pub type RendererConfig<'a, V> =
    irid_renderer::RendererConfig<'a, PerspectiveCamera, &'a str, &'a str, V, u16>;
