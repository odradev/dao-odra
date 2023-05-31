use std::thread::ThreadId;

use futures::{executor::{ThreadPool, ThreadPoolBuilder, block_on}, channel::mpsc, Stream, StreamExt, stream, future};

pub struct Executor {
    thread_pool: ThreadPool,
}

fn get() -> impl Stream<Item = (u32, ThreadId)> {
    let (tx, rx) = mpsc::channel(10);
    // let _cpu_pool = ThreadPoolBuilder::new()
    //     .pool_size(30)
    //     .after_start(move |_| {
    //         let id = std::thread::current().id();
    //         counter::increment();
    //         tx.clone().try_send((counter::value(), id)).unwrap()
    //     })
    //     .create()
    //     .unwrap();
    let pool = ThreadPool::new().unwrap();

    for _ in 0..10 {
        let tx = tx.clone();
        pool.spawn_ok(async move {
            let id = std::thread::current().id();
            counter::increment();
            tx.clone().try_send((counter::value(), id)).unwrap()
        });
    }

    stream::once(future::ready((0, std::thread::current().id()))).chain(rx)
}

fn get_sync() -> Vec<(u32, ThreadId)> {
    block_on(get().collect())
}

mod counter {
    use std::{cell::RefCell, thread_local};

    thread_local! {
        static NUMBER: RefCell<u32> = RefCell::new(0);
    }

    pub fn increment() {
        NUMBER.with(|n| {
            *n.borrow_mut() += 1;
        });
    }

    pub fn value() -> u32 {
        NUMBER.with(|n| n.borrow().clone())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_drop_after_start() {
        assert_eq!(super::get_sync(), vec![]);
        std::thread::sleep(std::time::Duration::from_millis(500)); // wait for background threads closed: https://github.com/rust-lang/miri/issues/1371
    }
}
