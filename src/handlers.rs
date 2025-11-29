use crate::{
    database::{create_paste, delete_paste, get_paste, increment_views},
    error::{AppError, NotFoundError},
    models::PasteForm,
    rendering::{render_form, render_paste},
};
use axum::{
    Form,
    extract::{Path, State},
    response::{Html, Redirect},
};
use chrono::{Duration, Utc};
use sqlx::{Pool, Sqlite};
use tracing::{debug, info};

pub async fn show_form() -> Html<String> {
    debug!("Serving the new paste form.");
    Html(render_form())
}

/// Handler for creation of a new paste
pub async fn create_paste_handler(
    State(pool): State<Pool<Sqlite>>,
    Form(form): Form<PasteForm>,
) -> Result<Redirect, AppError> {
    debug!(form = ?form, "Received new paste submission.");
    let id = create_paste(&pool, &form).await?;
    info!(paste_id = %id, "Successfully created new paste.");
    Ok(Redirect::to(&format!("/p/{}", id)))
}

/// Displays a paste by its ID
pub async fn view_paste_handler(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<String>,
) -> Result<Html<String>, AppError> {
    debug!(paste_id = %id, "Request to view paste.");
    let paste = get_paste(&pool, &id).await?;

    // Check if max_views is exceeded
    if let Some(max_views) = paste.max_views {
        if paste.views >= max_views {
            info!(paste_id = %id, "Paste has reached its max view count.");
            delete_paste(&pool, &id).await?;
            return Err(NotFoundError.into());
        }
    }

    // Check if the paste is expired
    if let (Some(expires_in_seconds), Some(created_at)) =
        (paste.expires_in_seconds, paste.created_at)
    {
        let expiration_time = created_at + Duration::seconds(expires_in_seconds);
        if Utc::now() > expiration_time {
            info!(paste_id = %id, "Paste has expired.");
            delete_paste(&pool, &id).await?;
            return Err(NotFoundError.into());
        }
    }

    // Increment views and render
    increment_views(&pool, &id).await?;
    debug!(paste_id = %id, "Paste is valid, rendering content.");

    Ok(Html(render_paste(&paste.content)))
}
