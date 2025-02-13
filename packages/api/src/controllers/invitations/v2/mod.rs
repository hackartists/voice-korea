use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, State},
        routing::{get, post},
        Extension, Json,
    },
};
use models::*;

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct InvitationControllerV2 {
    repo: InvitationRepository,
    user: UserRepository,
    member: OrganizationMemberRepository,
    // group_member: GroupMemberRepository,
}

impl InvitationControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Invitation::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let member = OrganizationMember::get_repository(pool.clone());
        let ctrl = InvitationControllerV2 { repo, user, member };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_invitation))
            .with_state(ctrl.clone())
            .route("/:id", get(Self::get_invitation)) //.post(Self::act_member_by_id))
            .with_state(ctrl.clone()))
    }

    pub async fn act_invitation(
        State(ctrl): State<InvitationControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<i64>,
        Json(body): Json<InvitationAction>,
    ) -> Result<Json<String>> {
        tracing::debug!("act_invitation {} {:?}", org_id, body);
        match body {
            InvitationAction::Invite(req) => Ok(Json(ctrl.invite_member(org_id, req).await?)),
        }
    }

    pub async fn get_invitation(
        State(_ctrl): State<InvitationControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<i64>,
    ) -> Result<Json<Invitation>> {
        tracing::debug!("get_invitation {:?}", id);
        Ok(Json(Invitation::default()))
    }
}

impl InvitationControllerV2 {
    pub async fn invite_member(&self, org_id: i64, req: InvitationInviteRequest) -> Result<String> {
        tracing::debug!("invite_member {} {:?}", org_id, req);
        let user = match self
            .user
            .find_one(&UserReadAction::new().find_by_email(req.email))
            .await
        {
            Ok(user) => user,
            Err(_) => {
                return Ok("user not found".to_string());
                // self.invite.
            }
        };

        let member = self
            .member
            .insert(
                user.id,
                org_id,
                req.name.unwrap_or_else(|| user.email.to_string()),
                req.role,
                None,
            )
            .await?;

        let msg = format!("member created successfully: id = {}", member.id);

        // TODO: invite group
        // let group_members = self
        //     .gr
        Ok(msg)
    }
}
