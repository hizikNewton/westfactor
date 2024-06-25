pub enum BlogAppError{
    DBError(String),
    ActixError(String),
    NotFoundError(String),
}