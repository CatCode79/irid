#![warn(
absolute_paths_not_starting_with_crate,
box_pointers,
elided_lifetimes_in_paths,
explicit_outlives_requirements,
keyword_idents,
macro_use_extern_crate,
meta_variable_misuse,
missing_abi,
missing_copy_implementations,
missing_debug_implementations,
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
unsafe_code,
unsafe_op_in_unsafe_fn,
unstable_features,
unused_crate_dependencies,
unused_extern_crates,
unused_import_braces,
unused_lifetimes,
unused_qualifications,
unused_results,
variant_size_differences,
// We don't match on a reference, unless required.
clippy::pattern_type_mismatch,
)]

//= CAMERA =========================================================================================

pub trait Camera {
    //- Constructors -------------------------------------------------------------------------------

    /// Create a new camera given the window's width and height
    fn new(width: f32, height: f32) -> Self;

    ///
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32>;

    //- Getters ------------------------------------------------------------------------------------

    ///
    fn eye(&self) -> cgmath::Point3<f32>;

    ///
    fn target(&self) -> cgmath::Point3<f32>;

    ///
    fn up(&self) -> cgmath::Vector3<f32>;

    //- Setters ------------------------------------------------------------------------------------

    fn set_eye(&mut self, value: cgmath::Point3<f32>);

    ///
    fn add_to_eye(&mut self, value: cgmath::Vector3<f32>);

    ///
    fn sub_to_eye(&mut self, value: cgmath::Vector3<f32>);
}
