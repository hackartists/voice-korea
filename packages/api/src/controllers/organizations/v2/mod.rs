use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Query, State},
        routing::get,
        Extension, Json,
    },
};
use models::*;

#[derive(Clone, Debug)]
pub struct OrganizationControllerV2 {
    repo: OrganizationRepository,
}

impl OrganizationControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Organization::get_repository(pool.clone());

        let ctrl = OrganizationControllerV2 { repo };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::list_organization))
            .with_state(ctrl.clone()))
    }

    pub async fn list_organization(
        State(ctrl): State<OrganizationControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<OrganizationParam>,
    ) -> Result<Json<OrganizationGetResponse>> {
        tracing::debug!("list_organization {:?}", param);

        match param {
            OrganizationParam::Query(q) => {
                let organizations: QueryResponse<OrganizationSummary> = ctrl.repo.find(&q).await?;
                Ok(Json(OrganizationGetResponse::Query(organizations)))
            }
            OrganizationParam::Read(_action) => {
                // TODO: implement to return filtered organization
                Ok(Json(OrganizationGetResponse::Read(Organization::default())))
            }
        }
    }
}
