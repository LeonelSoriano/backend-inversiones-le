#[derive(Debug, Clone, Serialize)]
pub struct Invalid {
    #[serde(skip_serializing)]
    pub msg: String,
    #[serde(skip_serializing)]
    pub args: Vec<String>,
    #[serde(rename = "msg")]
	pub human_readable: String
}

#[derive(Debug, Clone, Serialize)]
pub struct MultipleInvalid {
    pub tag: String,
    pub invalids: Vec<Invalid>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Error(pub Vec<Invalid>);

#[derive(Debug, Clone, Serialize)]
pub struct MultipleError(pub Vec<MultipleInvalid>);
