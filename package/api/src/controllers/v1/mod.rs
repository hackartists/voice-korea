use attributes::AttributeControllerV1;
use by_axum::axum::Router;
use groups::GroupControllerV1;
// use members::MemberControllerV1;
// use metadatas::MetadataControllerV1;
use organizations::OrganizationControllerV1;
use panels::PanelControllerV1;
use public_opinions::PublicOpinionControllerV1;
// use public_surveys::PublicSurveyControllerV1;

mod attributes;
// mod auth;
mod groups;
// mod members;
// mod metadatas;
mod organizations;
mod panels;
mod public_opinions;
// mod public_surveys;
// mod search;
// mod survey;
// mod verification;

pub fn router() -> Router {
    Router::new()
        // .nest("/search", search::router())
        // .nest("/auth", auth::router())
        // .nest("/verification", verification::router())
        // .nest("/survey", survey::router())
        .nest("/groups", GroupControllerV1::router())
        // .nest("/members", MemberControllerV1::router())
        .nest("/opinions", PublicOpinionControllerV1::router())
        .nest("/organizations", OrganizationControllerV1::router())
        .nest("/attributes", AttributeControllerV1::router())
        .nest("/panels", PanelControllerV1::router())
        // .nest("/metadata", MetadataControllerV1::router())
        // .nest("/surveys", PublicSurveyControllerV1::router())
}
