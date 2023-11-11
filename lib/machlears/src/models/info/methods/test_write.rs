use super::super::entity::Info;

impl Info {


    pub(crate) fn test_write() -> Info {
        Info {
            msg: String::from("Test message here!")
        }
    }

}