use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::{get, post},
        Extension, Json, Router,
    },
    log::root,
};
use easy_dynamodb::error::DynamoException;
use slog::o;

use crate::{
    common::CommonQueryResponse, middleware::auth::authorization_middleware,
};

use models::prelude::*;

#[derive(Clone, Debug)]
pub struct MemberControllerV1 {
    log: slog::Logger,
}

impl MemberControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "MemberControllerV1"));
        let ctrl = MemberControllerV1 { log };

        //FIXME: implement projects api
        Router::new()
            .route(
                "/",
                post(Self::act_member).get(Self::list_organization_members),
            )
            .route(
                "/:member_id",
                post(Self::act_member_by_id).get(Self::get_organization_member),
            )
            .route("/search/project", get(Self::search_projects))
            .route("/search", get(Self::search_members))
            .route("/invite", post(Self::invite_member))
            .layer(middleware::from_fn(authorization_middleware)) //FIXME: fix management authorization (오직 관리자만 해당 함수들 호출할 수 있도록 수정)
            .with_state(ctrl.clone())
    }

    pub async fn act_member(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<MemberControllerV1>,
        Json(body): Json<MemberActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_member"));
        slog::debug!(log, "act_member: {:?} {:?}", organization_id, body.clone());

        match body {
            MemberActionRequest::Create(req) => {
                let user_id = match find_user_id_by_email(req.email.clone()).await? {
                    Some(user_id) => user_id,
                    None => return Err(ApiError::NotFound),
                };

                let res: CommonQueryResponse<OrganizationMember> = CommonQueryResponse::query(
                    &log,
                    "gsi1-index",
                    None,
                    Some(100),
                    vec![("gsi1", OrganizationMember::get_gsi1(&""))], // FIXME: fix query
                )
                .await?;

                let cli = easy_dynamodb::get_client(&log);

                let member = if res.items.len() != 0 {
                    let item = res.items.first().unwrap();

                    if item.deleted_at.is_some() {
                        let mem: OrganizationMember = (
                            CreateMemberRequest {
                                name: None,
                                group: None,
                                role: None,
                                email: req.email.clone(),
                                projects: None,
                            },
                            item.id.clone(),
                            user_id.clone(),
                            organization_id.clone(),
                        )
                            .into();

                        mem
                    } else {
                        return Err(ApiError::AlreadyExists);
                    }
                } else {
                    let id = uuid::Uuid::new_v4().to_string();
                    let mem: OrganizationMember = (req.clone(), id, user_id.clone(), organization_id.clone()).into();

                    mem
                };

                match cli.upsert(member.clone()).await {
                    Ok(()) => {
                        if let Some(group) = req.group.clone() {
                            let _ = ctrl
                                .update_group_member(group.id, group.name, member.id.clone())
                                .await?;
                        }
                        return Ok(());
                    }

                    Err(e) => {
                        slog::error!(log, "Create Group Failed {e:?}");
                        return Err(ApiError::DynamoCreateException(e.to_string()));
                    }
                }
            }
        }
    }

    //TODO: implement act member by organization id (deprecated)
    pub async fn act_member_by_id(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<MemberControllerV1>,
        Path(member_id): Path<String>,
        Json(body): Json<MemberByIdActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_member"));
        slog::debug!(log, "act_member: {:?} {:?}", organization_id, member_id);

        match body {
            MemberByIdActionRequest::Update(req) => {
                ctrl.update_member(&member_id, req).await?;
            }
            MemberByIdActionRequest::Delete => {
                ctrl.remove_member(&member_id).await?;
            }
            MemberByIdActionRequest::AddProject(req) => {
                ctrl.add_project(&member_id, req).await?;
            }
            MemberByIdActionRequest::RemoveProject(project_id) => {
                ctrl.remove_project(&member_id, &project_id).await?;
            }
        }

        Ok(())
    }

    pub async fn invite_member(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<MemberControllerV1>,
        Json(body): Json<InviteMemberRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "invite_member"));
        let cli = easy_dynamodb::get_client(&log);
        slog::debug!(log, "invite_member: {:?} {:?}", organization_id, body);

        let res: CommonQueryResponse<User> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", User::gsi1(body.email.clone()))],
        )
        .await?;

        if res.items.len() == 0 {
            return Err(ApiError::NotFound);
        }

        let user = match res.items.first() {
            Some(user) => {
                if user.deleted_at.is_some() {
                    return Err(ApiError::NotFound);
                }
                user
            }
            None => return Err(ApiError::NotFound),
        };

        let user_id = user.id.clone();

        // check org member exists
        let res: CommonQueryResponse<OrganizationMember> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", OrganizationMember::get_gsi2(&user_id.clone(), &organization_id.clone()))],
        )
        .await?;

        if res.items.len() != 0 {
            return Err(ApiError::AlreadyExists);
        }

        let id = uuid::Uuid::new_v4().to_string();

        let member: OrganizationMember = (
            CreateMemberRequest {
                name: Some(body.name.clone()),
                group: body.group.clone(),
                role: body.role,
                email: body.email.clone(),
                projects: None,
            },
            id.clone(),
            user_id.clone(),
            organization_id.clone(),
        )
            .into();

        match cli.upsert(member.clone()).await {
            Ok(()) => {
                if let Some(group) = body.group.clone() {
                    let _ = ctrl
                        .update_group_member(group.id, group.name, id.to_string())
                        .await?;
                }

                Ok(())
            }
            Err(e) => {
                slog::error!(log, "Create Member Failed {e:?}");
                Err(ApiError::DynamoCreateException(e.to_string()))
            }
        }
    }

    //TODO: implement search projects in organization
    pub async fn search_projects(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<MemberControllerV1>,
        Query(params): Query<SearchParams>,
    ) -> Result<Json<CommonQueryResponse<MemberProject>>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "search_projects"));
        slog::debug!(log, "search_projects {:?} {:?}", organization_id, params);

        Ok(Json(CommonQueryResponse {
            items: vec![],
            bookmark: None,
        }))
    }

    pub async fn search_members(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<MemberControllerV1>,
        Query(params): Query<SearchParams>,
    ) -> Result<Json<CommonQueryResponse<OrganizationMember>>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "search_member"));
        slog::debug!(log, "search_member {:?} {:?}", organization_id, params);

        Ok(Json(CommonQueryResponse {
            items: vec![],
            bookmark: None,
        }))
    }

    pub async fn list_organization_members(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<MemberControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<ListMemberResponse>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "list_members"));
        slog::debug!(log, "list_members {:?} {:?}", organization_id, pagination);
        let cli = easy_dynamodb::get_client(&log);

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

        let res: CommonQueryResponse<OrganizationMember> = CommonQueryResponse::query(
            &log,
            "type-index",
            bookmark,
            size,
            vec![("type", "organization#member")],
        )
        .await?;

        let filtered = res
            .items
            .into_iter()
            .filter(|v| v.organization_id == organization_id && v.deleted_at.is_none())
            .collect::<Vec<OrganizationMember>>();

        let mut role_count = vec![0, 0, 0, 0, 0, 0]; //[전체, 관리자, 공론 관리자, 분석가, 중계자, 강연자]

        filtered.iter().for_each(|v| {
            if let Some(role) = &v.role {
                match role {
                    Role::Admin => role_count[1] += 1,
                    Role::PublicAdmin => role_count[2] += 1,
                    Role::Analyst => role_count[3] += 1,
                    Role::Mediator => role_count[4] += 1,
                    Role::Speaker => role_count[5] += 1,
                }
            }
            role_count[0] += 1;
        });

        let mut result: Vec<GroupMemberRelationship> = Vec::new();

        for item in filtered.iter() {
            let mut groups: Vec<Group> = Vec::new();

            let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
                &log,
                "gsi2-index",
                None,
                Some(100),
                vec![("gsi2", GroupMember::get_gsi2(&item.id.clone()))],
            )
            .await?;

            for item in res.items {
                if item.deleted_at.is_some() {
                    continue;
                }

                let group = cli
                    .get::<Group>(&item.group_id)
                    .await
                    .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?;

                if group.is_none() {
                    continue;
                }

                if group.clone().unwrap().deleted_at.is_some() {
                    continue;
                }

                groups.push(group.unwrap());
            }

            result.push(GroupMemberRelationship {
                member: item.clone(),
                groups,
                project: vec![], // TODO: implement projects
            });
        }

        Ok(Json(ListMemberResponse {
            members: result,
            role_count,
            bookmark: res.bookmark,
        }))
    }

    pub async fn get_organization_member(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<MemberControllerV1>,
        Path(member_id): Path<String>,
    ) -> Result<Json<GroupMemberRelationship>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "get_member"));
        slog::debug!(log, "get_member {:?} {:?}", organization_id, member_id);
        let cli = easy_dynamodb::get_client(&log);

        let res = cli
            .get::<OrganizationMember>(&member_id)
            .await
            .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?;

        if res.is_none() {
            return Err(ApiError::NotFound);
        }

        let member = res.unwrap();

        let mut groups: Vec<Group> = Vec::new();

        let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
            &log,
            "gsi2-index",
            None,
            Some(100),
            vec![("gsi2", GroupMember::get_gsi2(&member.id.clone()))],
        )
        .await?;

        for item in res.items {
            if item.deleted_at.is_some() {
                continue;
            }

            let group = cli
                .get::<Group>(&item.group_id)
                .await
                .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?;

            if group.is_none() {
                continue;
            }

            if group.clone().unwrap().deleted_at.is_some() {
                continue;
            }

            groups.push(group.unwrap());
        }

        Ok(Json(GroupMemberRelationship {
            member,
            groups,
            project: vec![], // TODO: implement projects
        }))
        
    }
}

impl MemberControllerV1 {
    pub async fn remove_group_member(
        &self,
        member_id: String
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update_member"));
        slog::debug!(log, "update_group_member");
        let cli = easy_dynamodb::get_client(&log);

        //check member
        let res = cli
            .get::<GroupMember>(&member_id)
            .await
            .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?;

        if res.is_none() {
            return Err(ApiError::NotFound);
        }

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

    pub async fn update_group_member(
        &self,
        group_id: String,
        group_name: String,
        member_id: String,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update_member"));
        slog::debug!(log, "update_group_member");
        let cli = easy_dynamodb::get_client(&log);

        //check member
        let res = cli
            .get::<OrganizationMember>(&member_id)
            .await
            .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?;

        if res.is_none() {
            return Err(ApiError::NotFound);
        }

        let member = res.unwrap();

        let res = cli
            .get::<User>(&member.user_id)
            .await
            .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?;

        if res.is_none() {
            return Err(ApiError::NotFound);
        }

        let user = res.unwrap();

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
            let group_member = GroupMember::new(id, group_id, member.organization_id.clone());

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
                    GroupMember::new(item.id.clone(), group_id, member.organization_id.clone());

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
                        &member.id.clone(),
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

impl MemberControllerV1 {
    pub async fn add_project(&self, member_id: &str, req: MemberProject) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "add_project"));
        slog::debug!(log, "add_project {:?} {:?}", member_id, req);
        Ok(())
    }

    pub async fn remove_project(&self, member_id: &str, project_id: &str) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove_project"));
        slog::debug!(log, "remove_project {:?} {:?}", member_id, project_id);
        Ok(())
    }

    pub async fn update_member(
        &self,
        member_id: &str,
        req: UpdateMemberRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update_member"));
        slog::debug!(log, "update_member");
        let cli = easy_dynamodb::get_client(&log);

        let now = chrono::Utc::now().timestamp_millis();

        let mut update_data: Vec<(&str, UpdateField)> = vec![];

        if req.name.is_some() {
            update_data.push(("name", UpdateField::String(req.name.unwrap())));
        }
        if req.group.is_some() {
            update_data.push((
                "group",
                UpdateField::String(req.group.clone().unwrap().name),
            ));
        }
        if req.role.is_some() {
            update_data.push(("role", UpdateField::String(req.role.unwrap())));
        }

        if update_data.len() != 0 {
            update_data.push(("updated_at", UpdateField::I64(now)));
        }

        let res = cli.update(member_id, update_data).await;

        match res {
            Ok(()) => {
                if req.group.is_some() {
                    let _ = self
                        .update_group_member(
                            req.group.clone().unwrap().id,
                            req.group.unwrap().name,
                            member_id.to_string(),
                        )
                        .await?;
                }

                Ok(())
            }
            Err(e) => {
                slog::error!(log, "Member Update Failed {e:?}");
                Err(ApiError::DynamoUpdateException(e.to_string()))
            }
        }
    }

    pub async fn remove_member(&self, member_id: &str) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove_member"));
        slog::debug!(log, "remove member {:?}", member_id);
        let cli = easy_dynamodb::get_client(&log);
        let now = chrono::Utc::now().timestamp_millis();

        let d: Result<Option<OrganizationMember>, DynamoException> = match cli.get(member_id).await {
            Ok(v) => Ok(v),
            Err(e) => {
                slog::error!(log, "Member Query Failed {e:?}");
                return Err(ApiError::DynamoQueryException(e.to_string()));
            }
        };

        if d.clone().unwrap().is_none() {
            return Err(ApiError::NotFound);
        }

        let res = cli
            .update(
                member_id,
                vec![
                    ("deleted_at", UpdateField::I64(now)),
                    ("type", UpdateField::String(OrganizationMember::get_deleted_type())),
                    (
                        "gsi1",
                        UpdateField::String(OrganizationMember::get_gsi1_deleted(&d.unwrap().unwrap().email)),
                    ),
                ],
            )
            .await;

        match res {
            Ok(()) => {
                let _ = self.remove_group_member(member_id.to_string()).await?;
                Ok(())
            }
            Err(e) => {
                slog::error!(log, "Remove Member Failed {e:?}");
                Err(ApiError::DynamoUpdateException(e.to_string()))
            }
        }
    }

    pub async fn create_member(
        user_id: String,
        org_id:String, 
        email: String,
        name: Option<String>,
        role: Option<Role>,
    ) -> Result<(), ApiError> {
        let log = root();
        let cli = easy_dynamodb::get_client(&log);

        let organization_member: OrganizationMember =
            OrganizationMember::new(
                user_id.clone(), 
                org_id.clone(), 
                email.clone(),
                name.clone(),
                role,
            );

        match cli.upsert(organization_member.clone()).await {
            Ok(()) => Ok(()),
            Err(e) => {
                slog::error!(log, "Create Organization Member Failed {e:?}");
                Err(ApiError::DynamoCreateException(e.to_string()))
            }
        }
    }
}

pub async fn find_user_id_by_email(
    email: String,
) -> Result<Option<String>, ApiError> {
    let log = root();

    let res: CommonQueryResponse<User> = CommonQueryResponse::query(
        &log,
        "gsi1-index",
        None,
        Some(1),
        vec![("gsi1", User::gsi1(email.clone()))],
    )
    .await?;

    if res.items.len() == 0 {
        return Ok(None);
    }

    let user = res.items.first().unwrap();

    if user.deleted_at.is_some() {
        return Ok(None);
    }

    Ok(Some(user.id.clone()))
}
