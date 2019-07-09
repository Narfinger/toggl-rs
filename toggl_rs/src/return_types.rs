/// The base type for all returned data
#[derive(Deserialize, Debug)]
pub struct Return<T> {
    data: T,
}

/// The Inner Type for the return from StartEntryCall
#[derive(Deserialize, Debug)]
pub struct StartEntryReturnInner {
    id: i64,
    pid: i64,
    wid: i64,
    billable: bool,
    start: chrono::DateTime<chrono::Utc>,
    tags: Option<Vec<String>>,
    duration: i64,
    description: String,
}

pub type StartEntryReturn = Return<StartEntryReturnInner>;

//yes they seem to be the same
pub type StopEntryReturn = Return<StartEntryReturnInner>;