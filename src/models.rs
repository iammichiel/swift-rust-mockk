

#[derive(Serialize)]
pub struct Endpoint {
    #[serde(rename = "adminURL")]
    pub admin_url: String,
    #[serde(rename = "internalURL")]
    pub internal_url: String,
    #[serde(rename = "publicURL")]
    pub public_url: String,
    pub region: String,
    pub id: String

}

#[derive(Serialize)]
pub struct Service {
    pub endpoints: Vec<Endpoint>,
    pub endpoints_links: Vec<String>,
    pub service_type: String,
    pub name: String
}

#[derive(Serialize)]
pub struct ServiceCatalog {
    pub endpoints: Vec<Service>
}