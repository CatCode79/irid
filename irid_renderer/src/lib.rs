#![warn(
absolute_paths_not_starting_with_crate,
box_pointers,
elided_lifetimes_in_paths,
explicit_outlives_requirements,
keyword_idents,
macro_use_extern_crate,
meta_variable_misuse,
missing_abi,
//missing_copy_implementations,
//missing_debug_implementations,
//missing_docs,
non_ascii_idents,
noop_method_call,
pointer_structural_match,
rust_2021_incompatible_closure_captures,
rust_2021_incompatible_or_patterns,
rust_2021_prefixes_incompatible_syntax,
rust_2021_prelude_collisions,
single_use_lifetimes,
trivial_casts,
trivial_numeric_casts,
unreachable_pub,
//unsafe_code,
unsafe_op_in_unsafe_fn,
unstable_features,
unused_crate_dependencies,
unused_extern_crates,
unused_import_braces,
unused_lifetimes,
unused_qualifications,
unused_results,
//variant_size_differences,
// We don't match on a reference, unless required.
clippy::pattern_type_mismatch,
)]

//= USES ===========================================================================================

pub use self::camera::*;
pub use self::pipeline::*;
pub use self::renderer::*;
pub use self::shader::*;
pub use self::utils::*;

//= MODS ===========================================================================================

// Exposed externally through the uses above
pub(crate) mod camera;
pub(crate) mod pipeline;
pub(crate) mod renderer;
pub(crate) mod shader;

// Used only internally
mod adapter;
mod camera_bind;
mod device;
mod instance;
mod queue;
mod surface;
mod texture_metadatas;
mod utils;
