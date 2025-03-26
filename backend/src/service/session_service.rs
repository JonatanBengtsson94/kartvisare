pub struct SessionService<S: SessionStore> {
    store: S,
}

impl<S: SessionStore> SessionService<S> {
    pub fn new(store: S) -> Self {
        SessionService { store }
    }

    pub async fn create_session(&self, user_id: i32) -> Result<(), Error> {
        // TODO: Create a session in the store
    }

    pub async fn get_session(&self, session_id: &str) -> Result<i32, Error> {
        // TODO: Get user id for session from the store
    }
}
