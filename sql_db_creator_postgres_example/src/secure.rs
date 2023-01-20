pub struct Credentials {
    pub user: String,
    pub password: String,
    pub host: String
}

impl Credentials {
    pub fn new() -> Self {
        // Credentials {
        //     user: String::from("root"),
        //     password: String::from("SEMFE_2003"),
        //     host: String::from("localhost")
        // }
        Credentials {
            user: String::from("postgres"),
            password: String::from("admin"),
            host: String::from("localhost")
        }
    }
}
