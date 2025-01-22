use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::post,
        Extension, Json, Router,
    },
    log::root,
};
use slog::o;

use crate::{
    common::CommonQueryResponse, middleware::auth::authorization_middleware,
};

use models::prelude::*;

#[derive(Clone, Debug)]
pub struct PublicOpinionControllerV1 {
    log: slog::Logger,
}

impl PublicOpinionControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "PublicOpinionControllerV1"));
        let ctrl = PublicOpinionControllerV1 { log };

        //TODO: implement metadata uri
        Router::new()
            .route("/", post(Self::act_attribute).get(Self::list_opinions))
            .route(
                "/:project_id",
                post(Self::act_opinion_by_id).get(Self::get_opinion),
            )
            .with_state(ctrl)
            .layer(middleware::from_fn(authorization_middleware))
    }

    pub async fn act_attribute(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PublicOpinionControllerV1>,
        Json(body): Json<OpinionActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_opinion"));
        slog::debug!(log, "act_opinion: {:?} {:?}", organization_id, body.clone());

        match body {
            OpinionActionRequest::Create(req) => {
                ctrl.create_opinion(&organization_id, req).await?;
            }
        }
        Ok(())
    }

    pub async fn act_opinion_by_id(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PublicOpinionControllerV1>,
        Path(project_id): Path<String>,
        Json(body): Json<OpinionByIdActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_opinion_by_id"));
        slog::debug!(
            log,
            "act_opinion_by_id: {:?} {:?}",
            organization_id,
            project_id
        );

        match body {
            OpinionByIdActionRequest::Delete => {
                ctrl.remove_opinion(&organization_id, &project_id).await?;
            }
            OpinionByIdActionRequest::Update(req) => {
                ctrl.update_opinion(&organization_id, &project_id, req)
                    .await?;
            }
            OpinionByIdActionRequest::UpdateProjectType(project_type) => {
                ctrl.update_project_type(&organization_id, &project_id, project_type)
                    .await?;
            }
            OpinionByIdActionRequest::UpdatePanels(panels) => {
                ctrl.update_panels(&organization_id, &project_id, panels)
                    .await?;
            }
            OpinionByIdActionRequest::UpdateStatus(status) => {
                ctrl.update_project_status(&organization_id, &project_id, status)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn get_opinion(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PublicOpinionControllerV1>,
        Path(project_id): Path<String>,
    ) -> Result<Json<OpinionResponse>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "get_opinion"));
        slog::debug!(log, "get_opinion: {:?} {:?}", organization_id, project_id);
        Ok(Json(OpinionResponse {
            project_id: "project id 1".to_string(),
            opinion_type: Field::Economy,
            project_name: "공론주제".to_string(),
            total_response_count: 60,
            response_count: 40,
            panels: vec![
                PanelInfo {
                    id: "1".to_string(),
                    name: "패널1".to_string(),
                },
                PanelInfo {
                    id: "2".to_string(),
                    name: "패널2".to_string(),
                },
                PanelInfo {
                    id: "3".to_string(),
                    name: "패널3".to_string(),
                },
            ],
            start_date: 1759244400,
            end_date: 1764601200,
            status: ProjectStatus::Finish,
        }))
    }

    pub async fn list_opinions(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PublicOpinionControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<OpinionResponse>>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "list_opinions"));
        slog::debug!(log, "list_opinions {:?} {:?}", organization_id, pagination);
        Ok(Json(CommonQueryResponse {
            items: vec![
                OpinionResponse {
                    project_id: "project id 1".to_string(),
                    opinion_type: Field::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::Finish,
                },
                OpinionResponse {
                    project_id: "project id 2".to_string(),
                    opinion_type: Field::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::Finish,
                },
                OpinionResponse {
                    project_id: "project id 3".to_string(),
                    opinion_type: Field::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::Ready,
                },
                OpinionResponse {
                    project_id: "project id 4".to_string(),
                    opinion_type: Field::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::Ready,
                },
                OpinionResponse {
                    project_id: "project id 5".to_string(),
                    opinion_type: Field::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::InProgress,
                },
                OpinionResponse {
                    project_id: "project id 6".to_string(),
                    opinion_type: Field::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::InProgress,
                },
            ],
            bookmark: None,
        }))
    }
}

impl PublicOpinionControllerV1 {
    pub async fn create_opinion(
        &self,
        organization_id: &str,
        body: CreateOpinionRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "create opinion"));
        slog::debug!(log, "create_opinion {:?} {:?}", organization_id, body);
        Ok(())
    }
}

impl PublicOpinionControllerV1 {
    pub async fn remove_opinion(
        &self,
        organization_id: &str,
        project_id: &str,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove opinion"));
        slog::debug!(log, "remove_opinion {:?} {:?}", organization_id, project_id);
        Ok(())
    }

    pub async fn update_opinion(
        &self,
        organization_id: &str,
        project_id: &str,
        body: UpdateOpinionRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update opinion"));
        slog::debug!(
            log,
            "update_opinion {:?} {:?} {:?}",
            organization_id,
            project_id,
            body
        );
        Ok(())
    }

    pub async fn update_project_type(
        &self,
        organization_id: &str,
        project_id: &str,
        project_type: Field,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update_project_type"));
        slog::debug!(
            log,
            "update_project_type {:?} {:?} {:?}",
            organization_id,
            project_id,
            project_type
        );
        Ok(())
    }

    pub async fn update_panels(
        &self,
        organization_id: &str,
        project_id: &str,
        panels: Vec<PanelInfo>,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update_panels"));
        slog::debug!(
            log,
            "update_panels {:?} {:?} {:?}",
            organization_id,
            project_id,
            panels
        );
        Ok(())
    }

    pub async fn update_project_status(
        &self,
        organization_id: &str,
        project_id: &str,
        status: ProjectStatus,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update_project_status"));
        slog::debug!(
            log,
            "update_project_status {:?} {:?} {:?}",
            organization_id,
            project_id,
            status
        );
        Ok(())
    }
}
