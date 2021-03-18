use async_std::task;
use futures::executor::block_on;
use std::time::Duration;

/// Table_result is an async function that polls from two future simultaneously and print the table in asynchronous manner.
///
/// #Arguments
///
/// Functions with no arguments.
///
/// #Return
///
/// No return type from function.
pub async fn table_result() {
    let take_table1 = async {
        for i in 1..=2 {
            let output = 2 * i;
            log::debug!("{}*{}={}", 2, i, output);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    let take_table2 = async {
        for i in 1..=2 {
            let output = 3 * i;
            log::debug!("{}*{}={}", 3, i, output);
        }
    };
    futures::join!(take_table1, take_table2);
}
/// Function block is blocking or terminating the thread at table_result.
///
/// #Arguments
///
/// No arguments in function.
///
/// #Return
///
/// No return from function.
pub fn block() {
    block_on(table_result());
}
