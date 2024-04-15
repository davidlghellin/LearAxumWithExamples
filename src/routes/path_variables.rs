use axum::extract::Path;

pub async fn path_variables(Path(id): Path<i32>) -> String {
    dbg!(id);
    id.to_string()
}

pub async fn hard_coded_patch() -> String {
    "Tienes 15!".to_owned()
}
