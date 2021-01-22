use actix_web::dev::ServiceRequest;
use actix_web::{get, App, Error, HttpResponse, HttpServer};

use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

use actix_web_grants::proc_macro::{has_any_role, has_permissions};
// All you need is just to `use` this trait
use actix_web_grants::permissions::AttachPermissions;

async fn validator(req: ServiceRequest, _credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    // Pass your `permissions`/`grants`/`permissions` here and you can use the `actix-web-grants`!
    req.attach(vec![_credentials.token().to_string()]);
    Ok(req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(auth)
            .service(admin_secured)
            .service(manager_secured)
    })
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
    .await
}

#[get("/admin")]
#[has_permissions("OP_GET_SECURED_INFO")]
// For the `ADMIN` - endpoint will give the HTTP status 200, otherwise - 403
// You can check via cURL:
// ```sh
// curl --location --request GET 'http://localhost:8080/admin' \
// --header 'Authorization: Bearer OP_GET_SECURED_INFO'
// ```
async fn admin_secured() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/manager")]
#[has_any_role("ADMIN", "MANAGER")]
// For the `ADMIN` or `MANAGER` - endpoint will give the HTTP status 200, otherwise - 403
// You can check via cURL:
// ```sh
// curl --location --request GET 'http://localhost:8080/manager' \
// --header 'Authorization: Bearer ROLE_MANAGER'
// ```
async fn manager_secured() -> HttpResponse {
    HttpResponse::Ok().finish()
}
