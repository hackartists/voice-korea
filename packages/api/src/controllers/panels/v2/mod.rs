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
            .route("/:id", post(Self::act_by_id).get(Self::get_panel))
            .with_state(ctrl.clone()))
    }

    pub async fn get_panel(
        State(ctrl): State<PanelControllerV2>,
        Path((org_id, id)): Path<(String, String)>,
        Extension(_auth): Extension<Option<Authorization>>,
    ) -> Result<Json<PanelV2>> {
        tracing::debug!("get_panel: {:?} {:?}", org_id, id);

        let panel = ctrl
            .repo
            .find_one(&PanelV2ReadAction::new().find_by_id(id))
            .await?;

        if panel.org_id != org_id {
            return Err(ApiError::Unauthorized);
        }

        Ok(Json(panel))
    }

    pub async fn act_by_id(
        State(ctrl): State<PanelControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((org_id, id)): Path<(String, String)>,
        Json(body): Json<PanelV2ByIdAction>,
    ) -> Result<Json<PanelV2>> {
        tracing::debug!("act_by_id: {:?} {:?}", id, body);

        match body {
            PanelV2ByIdAction::Update(params) => ctrl.update(org_id, id, params).await,
        }
    }

    pub async fn list_panels(
        State(ctrl): State<PanelControllerV2>,
        Path(org_id): Path<String>,
        Query(params): Query<PanelV2Query>,
    ) -> Result<Json<QueryResponse<PanelV2Summary>>> {
        tracing::debug!("list_panels: {:?}", params);

        let items = ctrl.repo.find(&params.with_org_id(org_id)).await?;
        Ok(Json(items))
    }

    pub async fn act_panel(
        State(ctrl): State<PanelControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<String>,
        Json(body): Json<PanelV2Action>,
    ) -> Result<Json<PanelV2>> {
        tracing::debug!("act_panel {:?}", body);

        match body {
            PanelV2Action::Delete(params) => ctrl.delete(params.id).await,
            PanelV2Action::Create(params) => ctrl.create(org_id, params).await,
        }
    }
}

impl PanelControllerV2 {
    pub async fn update(
        &self,
        org_id: String,
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
                    org_id: Some(org_id),
                },
            )
            .await?;

        Ok(Json(panel))
    }

    pub async fn delete(&self, panel_id: String) -> Result<Json<PanelV2>> {
        tracing::debug!("delete panel: {:?}", panel_id);

        let _ = self.repo.delete(&panel_id).await?;

        Ok(Json(PanelV2::default()))
    }

    pub async fn create(
        &self,
        org_id: String,
        params: PanelV2CreateRequest,
    ) -> Result<Json<PanelV2>> {
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
                org_id,
            )
            .await?;

        Ok(Json(panel))
    }
}
