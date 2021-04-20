use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub data: Option<Value>,
}

/*
impl<'r> Responder<'r> for Response {
    fn respond_to(self, _: &Request) -> Result<Res<'r>, Status> {
        print!(":D");
        Res::build()
            .header(ContentType::Plain)
            .ok()
    }
}
*/


#[derive(Debug)]
pub struct ResponseWithStatus {
    pub status_code: u16,
    pub response: Response,
}
