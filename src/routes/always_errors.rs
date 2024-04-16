use axum::http::StatusCode;

pub async fn always_errors() -> Result<(), StatusCode> {
    /*
    200 - 299 -> ok
    300 
    400 - 499 -> errores del cliente 
    500 - 599 -> errores server
    y hay uno q es soy una tetera (who knows)
    StatusCode::IM_A_TEAPOT
    */
    //Ok(());
    // 201 recurso creado con exito
    Err(StatusCode::IM_A_TEAPOT)
}
