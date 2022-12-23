pub use demonstration::routers::demonstration_router;
pub use management::routers::management_router;

pub mod common_handlers;
pub mod errors;

mod demonstration;
mod management;
pub mod shared_state;
mod transfer;
