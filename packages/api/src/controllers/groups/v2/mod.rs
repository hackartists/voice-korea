use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use models::*;

#[derive(Clone, Debug)]
pub struct GroupControllerV2 {
    repo: GroupRepository,
}

impl GroupControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Group::get_repository(pool);

        let ctrl = GroupControllerV2 { repo };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_group).post(Self::act_group_by_id))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_group).get(Self::list_group))
            .with_state(ctrl.clone()))
    }

    pub async fn act_group(
        State(_ctrl): State<GroupControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<GroupAction>,
    ) -> Result<Json<Group>> {
        tracing::debug!("act_group {:?}", body);
        Ok(Json(Group::default()))
    }

    pub async fn act_group_by_id(
        State(_ctrl): State<GroupControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<String>,
        Json(body): Json<GroupByIdAction>,
    ) -> Result<Json<Group>> {
        tracing::debug!("act_group_by_id {:?} {:?}", id, body);
        Ok(Json(Group::default()))
    }

    pub async fn get_group(
        State(_ctrl): State<GroupControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> Result<Json<Group>> {
        tracing::debug!("get_group {:?}", id);
        Ok(Json(Group::default()))
    }

    pub async fn list_group(
        State(_ctrl): State<GroupControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<GroupParam>,
    ) -> Result<Json<GroupGetResponse>> {
        tracing::debug!("list_group {:?}", q);
        Ok(Json(GroupGetResponse::Query(QueryResponse::default())))
    }
}
