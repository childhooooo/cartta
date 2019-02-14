pub mod account_test;
pub mod note_test;
mod helpers;

use parking_lot::Mutex;
static DB_LOCK: Mutex<()> = Mutex::new(());