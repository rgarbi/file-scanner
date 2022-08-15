pub mod background_hasher;
pub mod background_remover;
pub mod background_scanner;
pub mod background_scheduler;
pub mod queue_item;

pub static MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_WORK_AGAIN: i64 = 15;
