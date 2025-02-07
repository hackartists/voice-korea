use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::post,
        Extension, Json,
    },
};
use models::*;
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct PanelControllerV2 {
    repo: PanelV2Repository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PanelControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = PanelV2::get_repository(pool.clone());

        let ctrl = PanelControllerV2 { repo, pool };

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
            .find_one(&PanelV2ReadAction::new().find_by_id(id.parse::<i64>().unwrap()))
            .await?;

        if panel.org_id != org_id.parse::<i64>().unwrap() {
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
            PanelV2ByIdAction::Update(params) => {
                ctrl.update(
                    org_id.parse::<i64>().unwrap(),
                    id.parse::<i64>().unwrap(),
                    params,
                )
                .await
            }
        }
    }

    pub async fn list_panels(
        State(ctrl): State<PanelControllerV2>,
        Path(org_id): Path<String>,
        Query(params): Query<PanelV2Param>,
    ) -> Result<Json<PanelV2GetResponse>> {
        tracing::debug!("list_panels: {:?}", params);

        match params {
            PanelV2Param::Query(params) => match params.action {
                Some(PanelV2QueryActionType::SearchBy) => {
                    ctrl.search_by(params.with_org_id(org_id.parse::<i64>().unwrap()))
                        .await
                }
                _ => {
                    let items = ctrl
                        .repo
                        .find(&params.with_org_id(org_id.parse::<i64>().unwrap()))
                        .await?;

                    Ok(Json(PanelV2GetResponse::Query(items)))
                }
            },
            _ => Err(ApiError::InvalidAction),
        }
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
            PanelV2Action::Create(params) => {
                ctrl.create(org_id.parse::<i64>().unwrap(), params).await
            }
        }
    }
}

impl PanelControllerV2 {
    pub async fn search_by(
        &self,
        PanelV2Query {
            size,
            bookmark,
            org_id,
            name,
            ..
        }: PanelV2Query,
    ) -> Result<Json<PanelV2GetResponse>> {
        let mut total_count: i64 = 0;

        let query =
            PanelV2Summary::base_sql_with("where org_id = $1 and name ilike $2 limit $3 offset $4");
        tracing::debug!("search_by query: {}", query);

        let items: Vec<PanelV2Summary> = sqlx::query(&query)
            .bind(org_id.unwrap())
            .bind(format!("%{}%", name.unwrap()))
            .bind(size as i64)
            .bind(size as i64 * (bookmark.unwrap_or("1".to_string()).parse::<i64>().unwrap() - 1))
            .map(|r: PgRow| {
                use sqlx::Row;

                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(Json(PanelV2GetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }

    pub async fn update(
        &self,
        org_id: i64,
        panel_id: i64,
        params: PanelV2UpdateRequest,
    ) -> Result<Json<PanelV2>> {
        tracing::debug!("update panel: {:?}", params);

        let panel = self
            .repo
            .update(
                panel_id,
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

    pub async fn delete(&self, panel_id: i64) -> Result<Json<PanelV2>> {
        tracing::debug!("delete panel: {:?}", panel_id);

        let _ = self.repo.delete(panel_id).await?;

        Ok(Json(PanelV2::default()))
    }

    pub async fn create(&self, org_id: i64, params: PanelV2CreateRequest) -> Result<Json<PanelV2>> {
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
