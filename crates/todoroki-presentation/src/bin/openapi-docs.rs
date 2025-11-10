use todoroki_presentation::routes::ApiDocs;
use utoipa::OpenApi;

fn main() {
    let docs = generate_openapi_docs();
    print!("{docs}");
}

pub fn generate_openapi_docs() -> String {
    ApiDocs::openapi().to_yaml().unwrap()
}
