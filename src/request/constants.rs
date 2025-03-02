use lazy_static::lazy_static;
use reqwest::Client;

const BASE_URI: &str = "https://webappprd.acs.ncsu.edu/php/coursecat";
const SEARCH_ENDPOINT: &str = "/search.php";
const INDEX_ENDPOINT: &str = "/index.php";
pub const SPRING_TERM: u32 = 2251;

// Lazily initialize the Urls and Client
lazy_static! {
    pub static ref SEARCH_URL: String = format!("{}{}", BASE_URI, SEARCH_ENDPOINT);
    pub static ref INDEX_URL: String = format!("{}{}", BASE_URI, INDEX_ENDPOINT);

    pub static ref client: Client = Client::new();
}
