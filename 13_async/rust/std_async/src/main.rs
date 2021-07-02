use async_std::prelude::FutureExt;
use futures::future::join_all;
use futures::FutureExt as BoxedExt;

async fn a() -> u8 {
    println!("Hello ...");
    async_std::task::sleep(std::time::Duration::from_secs(3)).await;
    println!("waited");
    1u8
}

async fn b() -> u8 {
    println!("... World!");
    2u8
}

#[async_std::main]
async fn main() {
    a().await;
    b().await;
    println!("serial\n");

    let result = a().join(b()).await;
    println!("joined\n");
    assert_eq!(result, (1u8, 2u8));

    let result = a().race(b()).await; // often called `select`
    println!("raced {:?}\n", result);

    // `BoxedExt::boxed()` does `Box::pin(a()) as Pin<Box<dyn Future<Output = Foo>>>` for us
    let futures: Vec<_> = vec![b().boxed(), a().boxed(), b().boxed()];
    assert_eq!(join_all(futures).await, [2, 1, 2]);
    println!("joined vector\n");
}
