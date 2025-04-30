use actix_web::{web, HttpResponse};
use actix_multipart::Multipart;
use futures::TryStreamExt;
use crate::errors::AppError;
use crate::models::ai::{AIRequest, AIInput};
use crate::ai::service::AIServiceImpl;
use super::service::AIService;

pub async fn analyze_image(
    mut payload: Multipart,
    ai_service: web::Data<AIServiceImpl>,
) -> Result<HttpResponse, AppError> {
    let mut image = None;
    let mut prompt = None;
    let mut model = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_type = field.content_disposition();
        let name = content_type.get_name().ok_or_else(|| 
            AppError::ValidationError("Invalid form field".to_string()))?;

        match name {
            "image" => {
                let mut bytes = web::BytesMut::new();
                while let Some(chunk) = field.try_next().await? {
                    bytes.extend_from_slice(&chunk);
                }
                image = Some(bytes.freeze().to_vec());
            },
            "prompt" => {
                let mut text = String::new();
                while let Some(chunk) = field.try_next().await? {
                    text.push_str(std::str::from_utf8(&chunk)?);
                }
                prompt = Some(text);
            },
            "model" => {
                let mut text = String::new();
                while let Some(chunk) = field.try_next().await? {
                    text.push_str(std::str::from_utf8(&chunk)?);
                }
                model = Some(text);
            },
            _ => {}
        }
    }

    let image_data = image.ok_or_else(|| 
        AppError::ValidationError("Image is required".to_string()))?;
    
    log::debug!("Received image data length: {}", image_data.len());
    
    let request = AIRequest {
        input: AIInput::Image(image_data),
        prompt,
        model,
    };

    let response = ai_service.analyze(request).await?;
    Ok(HttpResponse::Ok().json(response))
}

pub async fn analyze_text(
    request: web::Json<AIRequest>,
    ai_service: web::Data<AIServiceImpl>,
) -> Result<HttpResponse, AppError> {
    let response = ai_service.analyze(request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}