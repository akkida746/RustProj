pub(crate) mod Types{
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize, FromForm, Responder)]
    pub(crate) struct User{
        pub name: String,
    }

}
