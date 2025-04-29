// src/services/admin.rs
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Constants
const GMT_TIME_OFFSET: i64 = 2 * 60 * 60 * 1000;

// Static variables to mimic the Kotlin implementation
static SIMULATED_TIME: AtomicI64 = AtomicI64::new(-1);
static UPDATED_TIME: AtomicI64 = AtomicI64::new(0);

// Update the simulated time
pub fn update_time(time: Option<i64>) {
    if let Some(t) = time {
        SIMULATED_TIME.store(t, Ordering::SeqCst);
    } else {
        SIMULATED_TIME.store(-1, Ordering::SeqCst);
    }
    
    UPDATED_TIME.store(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
        Ordering::SeqCst,
    );
}

// Get the current time, either real or simulated
pub fn now() -> i64 {
    let start = SIMULATED_TIME.load(Ordering::SeqCst);
    
    if start == -1 {
        // Use real time
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64
            + GMT_TIME_OFFSET
    } else {
        // Use simulated time with offset
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        let updated_time = UPDATED_TIME.load(Ordering::SeqCst);
        let offset = current_time - updated_time;
        
        start + offset
    }
}