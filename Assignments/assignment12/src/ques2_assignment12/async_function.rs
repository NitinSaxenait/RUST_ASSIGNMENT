use async_std::task;
use futures::executor::block_on;
use std::time::Duration;

/// task1 async function is using loop to generate data.
///
/// #Arguments
///
/// function is taking no arguments.
///
/// #Return
///
/// function is returning no value.
async fn task1() {
    for count in 0..=5 {
        log::debug!("This is data number {}", count);
        task::sleep(Duration::from_secs(1)).await;
    }
}
/// task2 async function is using loop to generate data.
///
/// #Arguments
///
/// function is taking no arguments.
///
/// #Return
///
/// function is returning no value.
async fn task2() {
    for index in 0..=5 {
        log::debug!("This is function2 data number {}", index);
    }
}
/// Function merge_tasks is taking two tasks and making join with future.
///
/// #Arguments
///
/// No arguments.
///
/// #Return
///
/// No return type in function.
async fn merge_tasks() {
    let take_function1 = task1();
    let take_function2 = task2();
    futures::join!(take_function1, take_function2);
}
pub fn mad() {
    block_on(merge_tasks());
}
