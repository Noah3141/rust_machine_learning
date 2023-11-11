use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]

pub(crate) struct Info {
    pub(crate) msg: String
}