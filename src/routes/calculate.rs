use crate::ec2pricing::*;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct CountRequest {
    count: u32,
}

pub async fn calculate(req: HttpRequest, info: Option<web::Query<CountRequest>>) -> HttpResponse {
    let mut count: u32 = 1;
    if let Some(x) = info {
        count = x.count;
    }
    let instance_type = req.match_info().get("instance_type").unwrap_or("anything");
    let instance_type = InstanceType::from_str(instance_type).unwrap();
    let configuration = get_configuration().expect("Failed to read configuration.");

    let mut response: Option<String> = None;
    for x in &configuration.instance_type_pricing {
        if instance_type == x.instance_type {
            response = Some(x.instance_pricing.json(count));
            break;
        }
    }

    match response {
        Some(r) => HttpResponse::Ok().content_type(ContentType::json()).body(r),
        None => HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("Something goes wrong... Have you specified unavailable instance type?"),
    }
}
