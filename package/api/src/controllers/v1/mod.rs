use attributes::AttributeControllerV1;
use by_axum::axum::Router;
// use members::MemberControllerV1;
// use metadatas::MetadataControllerV1;
use panels::PanelControllerV1;
use public_opinions::PublicOpinionControllerV1;
// use public_surveys::PublicSurveyControllerV1;

mod attributes;
// mod members;
// mod metadatas;
mod panels;
mod public_opinions;
// mod public_surveys;
// mod search;
// mod survey;
// mod verification;

pub fn router() -> Router {
    Router::new()
        // .nest("/search", search::router())
        // .nest("/verification", verification::router())
        // .nest("/survey", survey::router())
        // .nest("/members", MemberControllerV1::router())
        .nest("/opinions", PublicOpinionControllerV1::router())
        .nest("/attributes", AttributeControllerV1::router())
        .nest("/panels", PanelControllerV1::router())
        // .nest("/metadata", MetadataControllerV1::router())
        // .nest("/surveys", PublicSurveyControllerV1::router())
}
