// IKinder

// -----------------------------------------------------------------------------
// Arrays & Slices: Intro
// -----------------------------------------------------------------------------

async fn life() -> u32 {
    42
}

struct Connection;
struct ConnectionError(String);
struct Credentials;
struct CredentialsError(String);
struct Session;
struct SessionError(String);
pub struct ApiError(String);

impl From<SessionError> for ApiError {
    fn from(value: SessionError) -> Self {
        ApiError(value.0)
    }
}
impl From<CredentialsError> for ApiError {
    fn from(value: CredentialsError) -> Self {
        ApiError(value.0)
    }
}
impl From<ConnectionError> for ApiError {
    fn from(value: ConnectionError) -> Self {
        ApiError(value.0)
    }
}

async fn connect() -> Result<Connection, ConnectionError> {
    Ok(Connection)
}

async fn get_credentials(conn: &Connection) -> Result<Credentials, CredentialsError> {
    Ok(Credentials)
}

async fn generate_session(conn: &Connection, creds: &Credentials) -> Result<Session, SessionError> {
    Ok(Session)
}

#[tokio::main]
pub async fn main() -> Result<(), ApiError> {
    let future = life();
    let value = future.await;
    let value = life().await;

    let conn = connect().await?;
    let creds = get_credentials(&conn).await?;
    let session = generate_session(&conn, &creds).await?;
    Ok(())
}
