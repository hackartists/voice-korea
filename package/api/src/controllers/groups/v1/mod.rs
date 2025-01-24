use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::{get, post},
        Extension, Json, Router,
    },
    log::root,
};
use slog::o;

use crate::{
    common::CommonQueryResponse, controllers::members::v1::find_member_by_email, middleware::auth::authorization_middleware, utils::jwt::Claims
};

use models::prelude::*;

#[derive(Clone, Debug)]
pub struct GroupControllerV1 {
    log: slog::Logger,
}

// TODO: feat create group member
impl GroupControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "GroupControllerV1"));
        let ctrl = GroupControllerV1 { log };

        Router::new()
            .route("/", post(Self::act_group).get(Self::list_groups))
            .route(
                "/:group_id",
                post(Self::act_group_by_id).get(Self::get_group),
            )
            .route("/search", get(Self::search_groups))
            .route("/:group_id/members/search", get(Self::search_groups_by_id))
            .layer(middleware::from_fn(authorization_middleware)) //FIXME: fix management authorization
            .with_state(ctrl.clone())
    }

    pub async fn act_group(
        Extension(claims): Extension<Claims>,
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<GroupControllerV1>,
        Json(body): Json<GroupActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_group"));
        slog::debug!(log, "act_group: {:?} {:?}", organization_id, body.clone());

        match body {
            GroupActionRequest::Create(req) => {
                ctrl.create_group(req, organization_id, claims).await
            }
        }
    }

    //TODO: implement act group by organization id
    pub async fn act_group_by_id(
        Extension(claims): Extension<Claims>,
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<GroupControllerV1>,
        Path(group_id): Path<String>,
        Json(body): Json<GroupByIdActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_group"));
        slog::debug!(log, "act_group: {:?} {:?}", organization_id, group_id);

        match body {
            GroupByIdActionRequest::UpdateName(group_name) => {
                ctrl.update_group_name(&group_id, group_name).await?;
            }
            GroupByIdActionRequest::Delete => {
                ctrl.remove_group(&claims.id, &group_id).await?;
            }
            GroupByIdActionRequest::AddTeamMember(req) => {
                ctrl.add_team_member(&group_id, &organization_id, req).await?;
            }
            GroupByIdActionRequest::RemoveTeamMember(group_member_id) => {
                ctrl.remove_team_member(&group_id, &group_member_id).await?;
            }
        }

        Ok(())
    }

    //TODO: implement search groups by group id
    pub async fn search_groups_by_id(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<GroupControllerV1>,
        Path(group_id): Path<String>,
        Query(params): Query<SearchParams>,
    ) -> Result<Json<CommonQueryResponse<GroupResponse>>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "search_groups_by_id"));
        slog::debug!(
            log,
            "search_groups_by_id {:?} {:?} {:?}",
            organization_id,
            group_id,
            params
        );

        Ok(Json(CommonQueryResponse {
            items: vec![],
            bookmark: None,
        }))
    }

    //TODO: implement search groups
    pub async fn search_groups(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<GroupControllerV1>,
        Query(params): Query<SearchParams>,
    ) -> Result<Json<CommonQueryResponse<GroupResponse>>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "search_groups"));
        slog::debug!(log, "search_groups {:?} {:?}", organization_id, params);

        Ok(Json(CommonQueryResponse {
            items: vec![],
            bookmark: None,
        }))
    }

    pub async fn list_groups(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<GroupControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<GroupResponse>>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "list_groups"));
        let cli = easy_dynamodb::get_client(&log);
        slog::debug!(log, "list_groups {:?} {:?}", organization_id, pagination);

        let size = if let Some(size) = pagination.size {
            if size > 100 {
                Some(100)
            } else {
                Some(size as i32)
            }
        } else {
            Some(100)
        };

        let bookmark = pagination.bookmark;

        let res: CommonQueryResponse<Group> =
            CommonQueryResponse::query(&log, "type-index", bookmark, size, vec![("type", "group")])
                .await?;

        let mut groups: Vec<GroupResponse> = vec![];

        for group in res.items {
            if group.deleted_at.is_some() {
                continue;
            }

            if group.organization_id != organization_id {
                continue;
            }
            println!("hey {:?}", GroupMember::get_gsi1(&group.id));
            //FIXME: fix to parameter
            let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
                &log,
                "gsi1-index",
                None,
                Some(100),
                vec![("gsi1", GroupMember::get_gsi1(&group.id))],
            )
            .await?;
            let mut members: Vec<GroupMemberResponse> = vec![];

            for item in res.items {
                if item.deleted_at.is_some() {
                    continue;
                }

                if item.group_id != group.id {
                    continue;
                }

                let member: OrganizationMember = match cli
                    .get::<OrganizationMember>(&item.org_member_id)
                    .await
                    .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?
                {
                    Some(m) => m,
                    None => continue,
                };

                let user = match cli
                    .get::<User>(&member.user_id)
                    .await
                    .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?
                {
                    Some(u) => u,
                    None => continue,
                };

                members.push(GroupMemberResponse {
                    id: item.id,
                    created_at: item.created_at,
                    updated_at: item.updated_at,
                    deleted_at: item.deleted_at,
                    group_id: item.group_id,
                    org_member_id: item.org_member_id,
                    user_name: member.name.clone().unwrap_or_default(),
                    user_email: user.email.clone(),
                    role_name: member.role.clone().map(|r| r.to_string()),
                    group_name: group.name.clone(),
                });
            }

            groups.push(GroupResponse {
                id: group.id.clone(),
                creator: group.creator.clone(),
                created_at: group.created_at.clone(),
                updated_at: group.updated_at.clone(),
                deleted_at: group.deleted_at,
                name: group.name,
                members,
                // FIXME: implement projects api
                public_opinion_projects: vec![],
                investigation_projects: vec![],
            });
        }

        Ok(Json(CommonQueryResponse {
            items: groups,
            bookmark: res.bookmark,
        }))
    }

    pub async fn get_group(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<GroupControllerV1>,
        Path(group_id): Path<String>,
    ) -> Result<Json<GroupResponse>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "get_group"));
        slog::debug!(log, "get_group {:?} {:?}", organization_id, group_id);
        let cli = easy_dynamodb::get_client(&log);

        let group = match cli
            .get::<Group>(&group_id)
            .await
            .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?
        {
            Some(g) => g,
            None => return Err(ApiError::NotFound),
        };

        if !group.r#type.contains("deleted") {
            let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
                &log,
                "gsi1-index",
                None,
                Some(100),
                vec![("gsi1", GroupMember::get_gsi1(&group_id))],
            )
            .await?;
            let mut members: Vec<GroupMemberResponse> = vec![];
            for item in res.items {
                if item.deleted_at.is_some() {
                    continue;
                }

                let member = match cli
                    .get::<OrganizationMember>(&item.org_member_id)
                    .await
                    .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?
                {
                    Some(m) => m,
                    None => continue,
                };

                if member.deleted_at.is_some() {
                    continue;
                }
    
                let user = match cli
                    .get::<User>(&member.user_id)
                    .await
                    .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?
                {
                    Some(u) => u,
                    None => continue,
                };

                if item.deleted_at.is_some() {
                    continue;
                }

                members.push(GroupMemberResponse {
                    id: item.id,
                    created_at: item.created_at,
                    updated_at: item.updated_at,
                    deleted_at: item.deleted_at,
                    group_id: item.group_id,
                    org_member_id: item.org_member_id,
                    user_name: member.name.clone().unwrap_or_default(),
                    user_email: user.email.clone(),
                    role_name: member.role.clone().map(|r| r.to_string()),
                    group_name: group.name.clone(),
                });
            }
            Ok(Json(GroupResponse {
                id: group.id.clone(),
                creator: group.creator.clone(),
                created_at: group.created_at.clone(),
                updated_at: group.updated_at.clone(),
                deleted_at: group.deleted_at,
                name: group.name,
                members,
                // FIXME: implement projects api
                public_opinion_projects: vec![],
                investigation_projects: vec![],
            }))
        } else {
            Err(ApiError::NotFound)
        }
    }
}

// TODO: refactoring to group member model
impl GroupControllerV1 {
    pub async fn _remove_group_member(
        &self,
        ctrl: GroupControllerV1,
        member_id: String,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "update_member"));
        slog::debug!(log, "update_group_member");
        let cli = easy_dynamodb::get_client(&log);

        // check member in group
        let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
            &log,
            "gsi2-index",
            None,
            Some(1),
            vec![("gsi2", GroupMember::get_gsi2(&member_id))],
        )
        .await?;

        if res.items.len() == 0 {
            return Ok(());
        }

        let group_member = res.items.first().unwrap();
        let now = chrono::Utc::now().timestamp_millis();

        let res = cli
            .update(
                &group_member.id,
                vec![
                    ("deleted_at", UpdateField::I64(now)),
                    ("type", UpdateField::String(GroupMember::get_deleted_type())),
                    (
                        "gsi1",
                        UpdateField::String(GroupMember::get_gsi1_deleted(&group_member.group_id)),
                    ),
                    (
                        "gsi2",
                        UpdateField::String(GroupMember::get_gsi2_deleted(
                            &group_member.org_member_id,
                        )),
                    ),
                ],
            )
            .await;

        match res {
            Ok(()) => Ok(()),
            Err(e) => {
                slog::error!(log, "Remove Member Failed {e:?}");
                Err(ApiError::DynamoUpdateException(e.to_string()))
            }
        }
    }

    pub async fn upsert_group_member(
        &self,
        ctrl: GroupControllerV1,
        group_id: String,
        group_name: String,
        member_id: String,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "update_member"));
        slog::debug!(log, "update_group_member");
        let cli = easy_dynamodb::get_client(&log);

        //check member
        let member = match cli
            .get::<OrganizationMember>(&member_id)
            .await
            .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?
        {
            Some(m) => m,
            None => return Err(ApiError::NotFound),
        };

        let user = match cli
            .get::<User>(&member.user_id)
            .await
            .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?
        {
            Some(u) => u,
            None => return Err(ApiError::NotFound),
        };

        // check member in group
        let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
            &log,
            "gsi2-index",
            None,
            Some(1),
            vec![("gsi2", GroupMember::get_gsi2(&member_id))],
        )
        .await?;
        let now = chrono::Utc::now().timestamp_millis();

        if res.items.len() == 0 {
            //group member not exists
            let id = uuid::Uuid::new_v4().to_string();
            let group_member = GroupMember::new(id, group_id, member.id.clone());

            match cli.upsert(group_member.clone()).await {
                Ok(()) => {
                    let _ = cli
                        .update(
                            &member.id,
                            vec![
                                ("group", UpdateField::String(group_name)),
                                ("updated_at", UpdateField::I64(now)),
                            ],
                        )
                        .await
                        .map_err(|e| ApiError::DynamoUpdateException(e.to_string()));
                    return Ok(());
                }
                Err(e) => {
                    slog::error!(log, "Create Group Member Failed {e:?}");
                    return Err(ApiError::DynamoCreateException(e.to_string()));
                }
            }
        } else {
            //group member exists
            let item = res.items.first().unwrap();

            if item.deleted_at.is_some() {
                let group_member =
                    GroupMember::new(item.id.clone(), group_id, member.id.clone());

                match cli.upsert(group_member.clone()).await {
                    Ok(()) => {
                        let _ = cli
                            .update(
                                &member.id,
                                vec![
                                    ("group", UpdateField::String(group_name)),
                                    ("updated_at", UpdateField::I64(now)),
                                ],
                            )
                            .await
                            .map_err(|e| ApiError::DynamoUpdateException(e.to_string()));
                        return Ok(());
                    }
                    Err(e) => {
                        slog::error!(log, "Create Group Member Failed {e:?}");
                        return Err(ApiError::DynamoCreateException(e.to_string()));
                    }
                }
            } else {
                let mut update_data: Vec<(&str, UpdateField)> = vec![];
                let now = chrono::Utc::now().timestamp_millis();
                update_data.push((
                    "gsi1",
                    UpdateField::String(GroupMember::get_gsi1(&group_id)),
                ));
                update_data.push((
                    "gsi2",
                    UpdateField::String(GroupMember::get_gsi2(&member.id)),
                ));
                update_data.push(("group_id", UpdateField::String(group_id)));
                update_data.push(("org_member_id", UpdateField::String(member.id.clone())));
                update_data.push((
                    "user_name",
                    UpdateField::String(member.name.unwrap_or_default()),
                ));
                update_data.push(("user_email", UpdateField::String(user.email)));
                update_data.push(("updated_at", UpdateField::I64(now)));

                cli.update(&item.id, update_data)
                    .await
                    .map_err(|e| ApiError::DynamoUpdateException(e.to_string()))?;

                let _ = cli
                    .update(
                        &member.id,
                        vec![
                            ("group", UpdateField::String(group_name)),
                            ("updated_at", UpdateField::I64(now)),
                        ],
                    )
                    .await
                    .map_err(|e| ApiError::DynamoUpdateException(e.to_string()));
            }
        }
        Ok(())
    }
}

impl GroupControllerV1 {
    pub async fn add_team_member(
        &self,
        group_id: &str,
        org_id: &str,
        req: TeamMemberRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "add_team_member"));
        slog::debug!(log, "add_team_member {:?} {:?}", group_id, req);

        let cli = easy_dynamodb::get_client(&log);

        let member: CommonQueryResponse<OrganizationMember> = CommonQueryResponse::query(
            &log,
            "gsi2-index",
            None,
            Some(1),
            vec![("gsi2", OrganizationMember::get_gsi2(&req.email, org_id))],
        )
        .await?;

        if member.items.len() == 0 {
            return Err(ApiError::NotFound);
        }

        let member_id = member.items.first().unwrap().id.clone();

        let group = match cli
            .get::<Group>(&group_id)
            .await
            .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?
        {
            Some(g) => g,
            None => return Err(ApiError::NotFound),
        };

        // check whether the user is in the organization
        if group.organization_id != org_id {
            return Err(ApiError::InvalidPermissions);
        }

        // add member to group
        cli.create(GroupMember::new(
            uuid::Uuid::new_v4().to_string(),
            group_id.to_string(),
            member_id.clone(),
        ))
        .await
        .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

        // update member role
        if let Some(role) = req.role {
            cli.update(
                &member_id,
                vec![
                    ("role", UpdateField::String(role)),
                    ("updated_at", UpdateField::I64(chrono::Utc::now().timestamp_millis())),
                ],
            )
            .await
            .map_err(|e| ApiError::DynamoUpdateException(e.to_string()))?;
        }

        Ok(())
    }

    pub async fn remove_team_member(
        &self,
        group_id: &str,
        group_member_id: &str,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove_team_member"));
        slog::debug!(
            log,
            "remove_team_member {:?} {:?}",
            group_id,
            group_member_id
        );
        let cli = easy_dynamodb::get_client(&log);

        cli.update(
            group_member_id,
            vec![
                (
                    "updated_at",
                    UpdateField::I64(chrono::Utc::now().timestamp_millis()),
                ),
                (
                    "deleted_at",
                    UpdateField::I64(chrono::Utc::now().timestamp_millis()),
                ),
                ("type", UpdateField::String(GroupMember::get_deleted_type())),
            ],
        )
        .await
        .map_err(|e| ApiError::DynamoUpdateException(e.to_string()))?;

        Ok(())
    }

    pub async fn update_group_name(
        &self,
        group_id: &str,
        group_name: String,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update_group"));
        slog::debug!(log, "update_group_name {:?} {:?}", group_id, group_name);
        let cli = easy_dynamodb::get_client(&log);

        let now = chrono::Utc::now().timestamp_millis();

        let res = cli
            .update(
                group_id,
                vec![
                    ("updated_at", UpdateField::I64(now)),
                    ("name", UpdateField::String(group_name.clone())),
                ],
            )
            .await;

        match res {
            Ok(()) => {
                let mut bookmark = None;
                loop {
                    // remove member from group
                    let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
                        &log,
                        "gsi1-index",
                        bookmark,
                        Some(100),
                        vec![("gsi1", GroupMember::get_gsi1(&group_id))],
                    )
                    .await?;

                    for member in res.items {
                        let _ = cli
                            .update(
                                &member.org_member_id,
                                vec![
                                    ("updated_at", UpdateField::I64(now)),
                                    ("group", UpdateField::String(group_name.clone())),
                                ],
                            )
                            .await
                            .map_err(|e| ApiError::DynamoUpdateException(e.to_string()))?;
                    }

                    if res.bookmark.is_none() {
                        break;
                    }

                    bookmark = res.bookmark;
                }
                Ok(())
            }
            Err(e) => {
                slog::error!(log, "Group name Update Failed {e:?}");
                Err(ApiError::DynamoUpdateException(e.to_string()))
            }
        }
    }

    pub async fn remove_group(
        &self, 
        user_id: &str, 
        group_id: &str
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove group"));
        slog::debug!(log, "remove group {:?}", group_id);
        let cli = easy_dynamodb::get_client(&log);
        let now = chrono::Utc::now().timestamp_millis();

        let _ = cli
            .update(
                group_id,
                vec![
                    ("deleted_at", UpdateField::I64(now)),
                    ("type", UpdateField::String(Group::get_deleted_type())),
                    ("gsi1", UpdateField::String(Group::get_gsi_deleted(user_id))),
                ],
            )
            .await
            .map_err(|e| ApiError::DynamoUpdateException(e.to_string()));

        let mut bookmark = None;
        loop {
            // remove member from group
            let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
                &log,
                "gsi1-index",
                bookmark,
                Some(100),
                vec![("gsi1", GroupMember::get_gsi1(&group_id))],
            )
            .await?;

            for member in res.items {
                let _ = cli
                    .update(
                        &member.id,
                        vec![
                            ("deleted_at", UpdateField::I64(now)),
                            ("type", UpdateField::String(GroupMember::get_deleted_type())),
                            (
                                "gsi1",
                                UpdateField::String(GroupMember::get_gsi1_deleted(group_id)),
                            ),
                            (
                                "gsi2",
                                UpdateField::String(GroupMember::get_gsi2_deleted(
                                    &member.org_member_id,
                                )),
                            ),
                        ],
                    )
                    .await
                    .map_err(|e| ApiError::DynamoUpdateException(e.to_string()))?;

                let _ = cli
                    .update(
                        &member.org_member_id,
                        vec![("group", UpdateField::String("".to_string()))],
                    )
                    .await
                    .map_err(|e| ApiError::DynamoUpdateException(e.to_string()));
            }

            if res.bookmark.is_none() {
                break;
            }

            bookmark = res.bookmark;
        }

        Ok(())
    }

    pub async fn create_group(
        &self,
        req: CreateGroupRequest,
        organization_id: String,
        claims: Claims,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "create_group"));

        if req.name.trim().is_empty() {
            return Err(ApiError::ValidationError("Group name is required".to_string()));
        }

        slog::debug!(log, "create_group {:?}", req.clone());
        let cli = easy_dynamodb::get_client(&log);
        let id = uuid::Uuid::new_v4().to_string();
        let group: Group = (req.clone(), id.clone(), claims.id, organization_id.clone()).into();

        match cli.create(group.clone()).await {
            Ok(()) => {
                for member in req.members.clone() {
                    let member_id = match find_member_by_email(
                        member.member_email.clone(),
                        organization_id.clone(),
                    ).await {
                        Ok(m) => m.id,
                        Err(_) => {
                            slog::error!(log, "Member not found");
                            return Err(ApiError::NotFound);
                        }
                    };
                    self.upsert_group_member(
                        self.clone(),
                        id.clone(),
                        req.name.clone(),
                        member_id,
                    )
                    .await?;
                }

                return Ok(());
            }
            Err(e) => {
                slog::error!(log, "Create Group Failed {e:?}");
                return Err(ApiError::DynamoCreateException(e.to_string()));
            }
        };
    }
}
