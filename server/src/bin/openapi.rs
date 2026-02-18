use axumlive::ApiDoc;
use utoipa::OpenApi;

fn main() {
    print!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
}
