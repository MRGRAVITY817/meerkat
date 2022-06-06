use {axum::Router, proc_macro::TokenStream};

/// Controllers are responsible for handling incoming **requests** and returning **responses** to
/// client. A controller's purpose is to receive requests for the application.
///
/// Usage)
///
/// ```
/// use meerkat::{
///     controller::{controller, get, post, patch, delete},
///     extract::{Json, Path},
///     result::Result,
/// };
///
/// #[controller("cats")]
/// pub struct CatsController;
///
/// impl CatsController {
///     #[get(":id")]
///     pub async fn get_cat(Path(cat_id): Path<u32>) -> CatEntity {
///         format!("Cat no.{}", cat_id)
///     }
///
///     #[post("")]
///     pub async fn create_cat(Json<data>: Json(CreateCatDto)) -> Result<HttpResposne> {
///         true
///     }
///
///     #[patch(":id")]
///     pub async fn update_cat(Path(cat_id): Path<u32>, Json<data>: Json(UpdateCatDto)) -> bool {
///         true
///     }
///
///     #[delete(":id")]
///     pub async fn delete_cat(Path(cat_id): Path<u32>) -> bool {
///         true
///     }
/// }
///
/// pub struct CatEntity {
///     pub id: u32,
///     pub created_at: SystemTime,
///     pub updated_at: SystemTime,
///     pub gender: Gender,
///     pub name: String,
///     pub age: u32,
/// }
///
/// enum Gender {
///     Male,
///     Female,
/// }
///
/// pub struct CreateCatDto {
///     pub gender: Gender,
///     pub name: String,
///     pub age: u32,
/// }
///
/// pub struct UpdateCatDto {
///     pub gender: Gender,
///     pub name: String,
///     pub age: u32,
/// }
/// ```
#[proc_macro_attribute]
pub fn controller(_: TokenStream, _input: TokenStream) -> TokenStream {
    quote::quote!().into()
}
