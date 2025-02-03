use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use dto::*;

#[derive(Clone, Debug)]
pub struct OrganizationControllerV2 {
    repo: OrganizationRepository,
}

impl OrganizationControllerV2 {
    pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Organization::get_repository(pool);

        repo.create_table().await?;

        let ctrl = OrganizationControllerV2 { repo };

        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_Organization).post(Self::act_Organization_by_id),
            )
            .with_state(ctrl.clone())
            .route(
                "/",
                post(Self::act_Organization).get(Self::list_Organization),
            )
            .with_state(ctrl.clone()))
    }

    pub async fn act_Organization(
        State(_ctrl): State<OrganizationControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<OrganizationAction>,
    ) -> Result<Json<Organization>> {
        tracing::debug!("act_Organization {:?}", body);
        Ok(Json(Organization::default()))
    }

    pub async fn act_Organization_by_id(
        State(_ctrl): State<OrganizationControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<String>,
        Json(body): Json<OrganizationByIdAction>,
    ) -> Result<Json<Organization>> {
        tracing::debug!("act_Organization_by_id {:?} {:?}", id, body);
        Ok(Json(Organization::default()))
    }

    pub async fn get_Organization(
        State(_ctrl): State<OrganizationControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> Result<Json<Organization>> {
        tracing::debug!("get_Organization {:?}", id);
        Ok(Json(Organization::default()))
    }

    pub async fn list_Organization(
        State(_ctrl): State<OrganizationControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<OrganizationParam>,
    ) -> Result<Json<OrganizationGetResponse>> {
        tracing::debug!("list_Organization {:?}", q);
        Ok(Json(OrganizationGetResponse::Query(
            QueryResponse::default(),
        )))
    }
}
