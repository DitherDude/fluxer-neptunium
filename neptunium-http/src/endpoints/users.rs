#[cfg(feature = "user_api")]
mod delete_current_user_account;
#[cfg(feature = "user_api")]
mod disable_current_user_account;
#[cfg(feature = "user_api")]
mod forget_authorized_ips_for_current_user;
mod get_current_user_profile;
#[cfg(feature = "user_api")]
mod request_new_email_address;
#[cfg(feature = "user_api")]
mod request_replacement_email_for_bounced_address;
#[cfg(feature = "user_api")]
mod resend_new_email_confirmation;
#[cfg(feature = "user_api")]
mod resend_original_email_confirmation;
#[cfg(feature = "user_api")]
mod resend_replacement_email_code;
#[cfg(feature = "user_api")]
mod start_email_change;
#[cfg(feature = "user_api")]
mod update_current_user_profile;
#[cfg(feature = "user_api")]
mod verify_new_email_address;
#[cfg(feature = "user_api")]
mod verify_original_email_address;
#[cfg(feature = "user_api")]
mod verify_replacement_email_for_bounced_address;

#[cfg(feature = "user_api")]
pub use delete_current_user_account::*;
#[cfg(feature = "user_api")]
pub use disable_current_user_account::*;
#[cfg(feature = "user_api")]
pub use forget_authorized_ips_for_current_user::*;
pub use get_current_user_profile::*;
#[cfg(feature = "user_api")]
pub use request_new_email_address::*;
#[cfg(feature = "user_api")]
pub use request_replacement_email_for_bounced_address::*;
#[cfg(feature = "user_api")]
pub use resend_new_email_confirmation::*;
#[cfg(feature = "user_api")]
pub use resend_original_email_confirmation::*;
#[cfg(feature = "user_api")]
pub use resend_replacement_email_code::*;
#[cfg(feature = "user_api")]
pub use start_email_change::*;
#[cfg(feature = "user_api")]
pub use update_current_user_profile::*;
#[cfg(feature = "user_api")]
pub use verify_new_email_address::*;
#[cfg(feature = "user_api")]
pub use verify_original_email_address::*;
#[cfg(feature = "user_api")]
pub use verify_replacement_email_for_bounced_address::*;
