//! Build your own [block_on()]
//!
//! [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
use futures::channel::oneshot;
use std::thread;
use std::time::Duration;
use stjepang::blog20200125::v1::block_on;

fn main() {
    let (s, r) = oneshot::channel();

    // Create a sender thread.
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        s.send("Hello, world!").unwrap();
    });

    // Block the thread until the message is received.
    let msg = block_on(async {
        println!("Awaiting...");
        r.await.unwrap()
    });
    println!("{}", msg);
}
