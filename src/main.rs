use failure::Error;
use std::time::Duration;
use autoenroll::{Requester, on_change};

fn main() -> Result<(), Error> {
    let requester = Requester::new("http://classfind.stonybrook.edu/vufind/AJAX/JSON?method=getItemVUStatuses",
                                       &[("itemid", "90280"), ("strm", "1198")],
                                        &["data"]);
    let thread = on_change(requester, Duration::from_secs(10), |changes| {
        dbg!(changes);
    });

    thread.join().unwrap_err();
    Ok(())
}