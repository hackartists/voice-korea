use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::post,
        Extension, Json,
    },
};
use models::{
    panel_v2::{
        PanelV2, PanelV2Action, PanelV2ByIdAction, PanelV2CreateRequest, PanelV2Query,
        PanelV2ReadAction, PanelV2Repository, PanelV2Summary, PanelV2UpdateRequest,
    },
    *,
};

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
        tracing::debug!("act_by_id: {:?} {:?}", panel_id, body);

        match body {
            PanelV2ByIdAction::Update(params) => ctrl.update(params).await,
        }
    }

    pub async fn list_panels(
        State(ctrl): State<PanelControllerV2>,
        Query(params): Query<PanelV2Query>,
    ) -> Result<Json<QueryResponse<PanelV2Summary>>> {
        tracing::debug!("list_panels: {:?}", params);

        let items = ctrl.repo.find(&params).await?;

        let mut res: Vec<PanelV2Summary> = vec![];

        for item in items.items {
            res.push(PanelV2Summary {
                id: item.id,
                created_at: item.created_at,
                name: item.name,
                user_count: item.user_count,
                age: item.age,
                gender: item.gender,
                region: item.region,
                salary: item.salary,
                org_id: item.org_id,
            });
        }

        Ok(Json(QueryResponse {
            items: res,
            total_count: items.total_count,
        }))
    }

    pub async fn act_panel(
        State(ctrl): State<PanelControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<PanelV2Action>,
    ) -> Result<Json<PanelV2>> {
        tracing::debug!("act_panel {:?}", body);

        match body {
            PanelV2Action::Delete(params) => ctrl.delete(params.id).await,
            PanelV2Action::Create(params) => ctrl.create(params).await,
        }
    }
}

impl PanelControllerV2 {
    //FIXME: implement update panel logic when update method is implemented
    pub async fn update(&self, params: PanelV2UpdateRequest) -> Result<Json<PanelV2>> {
        tracing::debug!("update panel: {:?}", params);

        Ok(Json(PanelV2::default()))
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
