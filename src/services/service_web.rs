use crate::{
    constant::var_constant::{StrConfig, VarConstant},
    services::service_routes,
};

use axum::Router;
use tokio;

pub struct SrvWeb;

impl SrvWeb {
    pub async fn init() {
        let config: StrConfig = VarConstant::get_config();

        let mut app: Router = Router::new();

        // ===============================================================================
        // =========== WE STORE THE ROUTES IN A FUNCTION FOR EASIER DUPLICATE ============
        // =========== THATS WHY WE HAVE TO LOOP IT FIRST.                    ============
        // ===============================================================================

        let arr_routes: Vec<fn(Router) -> Router> = service_routes::SrvRoutes::get_routes();

        for el in &arr_routes {
            app = el(app);
        }

        let addr: String = config.addr;
        let port: String = config.port.to_string();

        let full_addr: String = format!("{addr}:{port}");

        let listener = match tokio::net::TcpListener::bind(full_addr).await {
            Ok(l) => l,
            Err(e) => {
                println!("Failed : {}", e);
                return;
            }
        };

        axum::serve(listener, app).await.expect("Failed")
    }
}
