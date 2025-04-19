/// Loads environment variables from a `.env` file into the application's environment.
/// This function should be called at the start of the application to ensure all necessary
/// environment variables are available. It uses the `dotenv` crate to parse the `.env` file.
pub fn setup_env() {
    dotenv::dotenv().ok();
}