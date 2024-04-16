use axum::Extension;

use super::SharedDate;

pub async fn middleware_message(Extension(shared_data):Extension<SharedDate>) ->String {
    shared_data.message
}