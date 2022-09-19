use std::sync::Arc;

use tokio::sync::Mutex;

pub type Shared<T> = Arc<Mutex<T>>;
pub fn new_shared<T>(sharing: T) -> Shared<T> {
    Arc::new(Mutex::new(sharing))
}
