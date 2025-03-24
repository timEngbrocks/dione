use std::{sync::mpsc::Receiver, thread, time::Duration};

use thread_pool::{ControlMessage, ThreadPool};

#[test]
fn test() {
    let mut pool = ThreadPool::default();

    let f = |_: (), rx: Receiver<ControlMessage>| {
        for _ in 1..=1000 {
            thread::sleep(Duration::from_millis(10));

            if matches!(rx.try_recv(), Ok(ControlMessage::SHUTDOWN)) {
                return;
            }
        }
    };
    pool.spawn((), f);
    pool.spawn((), f);
    pool.spawn((), f);

    assert!(pool.join_all().is_ok());
}
