use chrono::NaiveDateTime;
use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatedPromotionPutModel {
    #[garde(length(max = 255))]
    #[garde(alphanumeric)]
    pub title: Option<String>,
    #[garde(skip)]
    pub start_year: Option<chrono::NaiveDate>,
    #[garde(skip)]
    pub end_year: Option<chrono::NaiveDate>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatedProjectPutModel {
    #[garde(length(max = 64))]
    #[garde(alphanumeric)]
    pub name: Option<String>,
    #[garde(skip)]
    pub description: Option<String>,
    #[garde(skip)]
    pub start_date: Option<NaiveDateTime>,
    #[garde(skip)]
    pub end_date: Option<NaiveDateTime>,
    #[garde(skip)]
    pub notation_period_duration: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatedStudentPutModel {
    #[garde(length(max = 64))]
    #[garde(ascii)]
    pub name: Option<String>,
    #[garde(length(max = 64))]
    #[garde(ascii)]
    pub surname: Option<String>,
    #[garde(length(max = 128))]
    #[garde(email)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatedGroupPutModel {
    #[garde(length(max = 64))]
    #[garde(ascii)]
    pub name: Option<String>,
    #[garde(skip)]
    pub mark: Option<f64>,
}