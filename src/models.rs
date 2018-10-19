#[derive(Serialize)]
pub struct TokenRequestResponse {
    pub access: Access
}

#[derive(Serialize)]
pub struct Access {
    pub token: Token,
    #[serde(rename = "serviceCatalog")]
    pub service_catalog: Vec<Service>,
    pub user: User,
    pub metadata: Metadata
}

#[derive(Serialize)]
pub struct Token {
    pub issued_at: String,
    pub expires: String,
    pub id: String,
    pub tenant: TokenTenant,
}

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub name: String,
    pub roles_links: Vec<String>,
    pub roles: Vec<String>
}

#[derive(Serialize)]
pub struct TokenTenant {
    pub id: String,
    pub description: String,
    pub name: String,
    pub enabled: bool
}

#[derive(Serialize)]
pub struct Metadata {
    pub is_admin: bool,
    pub roles: Vec<String>
}

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
    #[serde(rename = "type")]
    pub service_type: String,
    pub name: String
}

#[derive(Serialize)]
pub struct ServiceCatalog {

}