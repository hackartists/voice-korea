use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::post,
        Extension, Json,
    },
};
use models::*;

#[derive(Clone, Debug)]
pub struct PanelControllerV2 {
    repo: PanelV2Repository,
}

impl PanelControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = PanelV2::get_repository(pool.clone());

        let ctrl = PanelControllerV2 { repo };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_panel).get(Self::list_panels))
            .route("/:panel_id", post(Self::act_by_id).get(Self::get_panel))
            .with_state(ctrl.clone()))
    }

    pub async fn get_panel(
        State(ctrl): State<PanelControllerV2>,
        Path(panel_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
    ) -> Result<Json<PanelV2>> {
        // TODO: check permission
        tracing::debug!("get_panel: {:?}", panel_id);

        let panel = ctrl
            .repo
            .find_one(&PanelV2ReadAction::new().find_by_id(panel_id))
            .await?;

        Ok(Json(panel))
    }

    pub async fn act_by_id(
        State(ctrl): State<PanelControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(panel_id): Path<String>,
        Json(body): Json<PanelV2ByIdAction>,
    ) -> Result<Json<PanelV2>> {
        // TODO: check permission
        tracing::debug!("act_by_id: {:?} {:?}", panel_id, body);

        match body {
            PanelV2ByIdAction::Update(params) => ctrl.update(panel_id, params).await,
        }
    }

    pub async fn list_panels(
        State(ctrl): State<PanelControllerV2>,
        Query(params): Query<PanelV2Query>,
    ) -> Result<Json<QueryResponse<PanelV2Summary>>> {
        // TODO: check permission
        tracing::debug!("list_panels: {:?}", params);

        let items = ctrl.repo.find(&params).await?;
        Ok(Json(items))
    }

    pub async fn act_panel(
        State(ctrl): State<PanelControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<PanelV2Action>,
    ) -> Result<Json<PanelV2>> {
        // TODO: check permission
        tracing::debug!("act_panel {:?}", body);

        match body {
            PanelV2Action::Delete(params) => ctrl.delete(params.id).await,
            PanelV2Action::Create(params) => ctrl.create(params).await,
        }
    }
}

impl PanelControllerV2 {
    pub async fn update(
        &self,
        panel_id: String,
        params: PanelV2UpdateRequest,
    ) -> Result<Json<PanelV2>> {
        tracing::debug!("update panel: {:?}", params);

        let panel = self
            .repo
            .update(
                &panel_id,
                PanelV2RepositoryUpdateRequest {
                    name: Some(params.name),
                    user_count: Some(params.user_count),
                    age: Some(params.age),
                    gender: Some(params.gender),
                    region: Some(params.region),
                    salary: Some(params.salary),
                    org_id: None,
                },
            )
            .await?;

        Ok(Json(panel))
    }

    //FIXME: implement delete panel logic when update method is implemented
    pub async fn delete(&self, panel_id: String) -> Result<Json<PanelV2>> {
        tracing::debug!("delete panel: {:?}", panel_id);

        Ok(Json(PanelV2::default()))
    }

    pub async fn create(&self, params: PanelV2CreateRequest) -> Result<Json<PanelV2>> {
        tracing::debug!("create panel: {:?}", params);

        let panel = self
            .repo
            .insert(
                params.name,
                params.user_count,
                params.age,
                params.gender,
                params.region,
                params.salary,
                params.org_id,
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to insert panel: {}", e);
                ApiError::DuplicateUser
            })?;

        Ok(Json(panel))
    }
}
