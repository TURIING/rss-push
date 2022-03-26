use jsonwebtoken::Algorithm;

// jwt encode secret
pub const JWTSECRET: &[u8] = b"turiing";
// jwt encode and decode algorithm
pub const JWTALG: Algorithm = Algorithm::HS512;
// database file path
pub const DATABASEURL: &str = "./database.sqlite";