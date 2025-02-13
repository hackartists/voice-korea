use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use models::*;

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct MemberControllerV2 {
    repo: OrganizationMemberRepository,
    user: UserRepository,
}

impl MemberControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = OrganizationMember::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let ctrl = MemberControllerV2 { repo, user };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_member).get(Self::list_member))
            .with_state(ctrl.clone())
            .route("/:id", get(Self::get_member)) //.post(Self::act_member_by_id))
            .with_state(ctrl.clone()))
    }

    pub async fn act_member(
        State(_ctrl): State<MemberControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<OrganizationMemberAction>,
    ) -> Result<Json<OrganizationMember>> {
        tracing::debug!("act_member {:?}", body);
        match body {
            OrganizationMemberAction::Create(_req) => {
                // if let member = ctrl.create_member(req, org_id)
                let member = OrganizationMember::default();
                Ok(Json(member))
            }
            OrganizationMemberAction::Update(_req) => {
                let member = OrganizationMember::default();
                Ok(Json(member))
            }
        }
    }

    // pub async fn act_member_by_id(
    //     State(_ctrl): State<OrganizationMemberControllerV1>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(id): Path<String>,
    //     Json(body): Json<OrganizationMemberByIdAction>,
    // ) -> Result<Json<OrganizationMember>> {
    //     tracing::debug!("act_member_by_id {:?} {:?}", id, body);
    //     Ok(Json(OrganizationMember::default()))
    // }

    pub async fn get_member(
        State(_ctrl): State<MemberControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<i64>,
    ) -> Result<Json<OrganizationMember>> {
        tracing::debug!("get_member {:?}", id);
        Ok(Json(OrganizationMember::default()))
    }

    pub async fn list_member(
        State(_ctrl): State<MemberControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<OrganizationMemberParam>,
    ) -> Result<Json<OrganizationMemberGetResponse>> {
        tracing::debug!("list_member {:?}", param);

        match param {
            OrganizationMemberParam::Query(_q) => {
                // let members = ctrl
                //     .repo
                //     .find(&OrganizationMemberQueryAction::new().find_by_organization_id(q.org_id))
                //     .await?;

                Ok(Json(OrganizationMemberGetResponse::Query(
                    QueryResponse::default(),
                )))
            }
        }
    }
}

impl MemberControllerV2 {
    // pub async fn get_member_by_id(
    //     State(_ctrl): State<MemberControllerV2>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(id): Path<i64>,
    // ) -> Result<Json<OrganizationMember>> {
    //     tracing::debug!("get_member_by_id {:?}", id);
    //     Ok(Json(OrganizationMember::default()))
    // }
}
