use futures::stream::{self, Stream, StreamExt};

pub struct Error;

fn my_stream() -> impl Stream<Item = Result<u8, Error>> {
    stream::repeat(1).map(Ok)
}

pub async fn foo_working() -> Result<u8, Error> {
    let mut s = my_stream();
    let mut acc = 0;
    for value in s.next().await {
        let value = value?;
        acc += value;
    }

    Ok(acc)
}

pub async fn foo_not_working() -> Result<u8, Error> {
    let mut s = my_stream();
    let mut acc = 0;
    // fails with  
    // the trait `std::convert::From<std::option::NoneError>` is not implemented for `Error`
    for value in s.next().await? {
        acc += value;
    }

    Ok(acc)
}
