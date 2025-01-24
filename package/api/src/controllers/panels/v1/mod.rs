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
    common::CommonQueryResponse, middleware::auth::authorization_middleware
};
use models::prelude::*;

#[derive(Clone, Debug)]
pub struct PanelControllerV1 {
    log: slog::Logger,
}

impl PanelControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "PanelControllerV1"));
        let ctrl = PanelControllerV1 { log };

        Router::new()
            .route("/", post(Self::act_panel).get(Self::list_panels))
            .route(
                "/:panel_id",
                post(Self::act_panel_by_id).get(Self::get_panel),
            )
            .with_state(ctrl)
            .layer(middleware::from_fn(authorization_middleware))
    }

    pub async fn act_panel(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PanelControllerV1>,
        Json(body): Json<PanelActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_panel"));
        slog::debug!(log, "act_panel: {:?} {:?}", organization_id, body);

        match body {
            PanelActionRequest::Create(req) => {
                ctrl.create_panel(&organization_id, req).await?;
            }
        }

        Ok(())
    }

    pub async fn act_panel_by_id(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PanelControllerV1>,
        Path(panel_id): Path<String>,
        Json(body): Json<PanelByIdActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_panel_by_id"));
        slog::debug!(log, "act_panel_by_id: {:?} {:?}", organization_id, panel_id);

        match body {
            PanelByIdActionRequest::Delete => {
                ctrl.remove_panel(&organization_id, &panel_id).await?;
            }
            PanelByIdActionRequest::Update(req) => {
                ctrl.update_panel(&organization_id, &panel_id, req).await?;
            }
        }

        Ok(())
    }

    pub async fn get_panel(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PanelControllerV1>,
        Path(panel_id): Path<String>,
    ) -> Result<Json<Panel>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "get_panel"));
        slog::debug!(log, "get_panel: {:?} {:?}", organization_id, panel_id);

        Ok(Json(Panel {
            id: "1".to_string(),
            name: "패널명1".to_string(),
            count: 50,
            attribute: vec![
                PanelAttributeItem {
                    id: Some("1".to_string()),
                    name: "직업".to_string(),
                    attribute: vec![
                        AttributeItem {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        AttributeItem {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
                PanelAttributeItem {
                    id: Some("2".to_string()),
                    name: "성별".to_string(),
                    attribute: vec![
                        AttributeItem {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        AttributeItem {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
                PanelAttributeItem {
                    id: Some("3".to_string()),
                    name: "나이".to_string(),
                    attribute: vec![
                        AttributeItem {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        AttributeItem {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
                PanelAttributeItem {
                    id: Some("4".to_string()),
                    name: "학력".to_string(),
                    attribute: vec![
                        AttributeItem {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        AttributeItem {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
                PanelAttributeItem {
                    id: Some("5".to_string()),
                    name: "거주지".to_string(),
                    attribute: vec![
                        AttributeItem {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        AttributeItem {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
                PanelAttributeItem {
                    id: Some("6".to_string()),
                    name: "국적".to_string(),
                    attribute: vec![
                        AttributeItem {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        AttributeItem {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
            ],
        }))
    }

    pub async fn list_panels(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PanelControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<Panel>>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "list_panels"));
        slog::debug!(log, "list_panels {:?} {:?}", organization_id, pagination);

        Ok(Json(CommonQueryResponse {
            items: vec![
                Panel {
                    id: "1".to_string(),
                    name: "패널명1".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeItem {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                Panel {
                    id: "2".to_string(),
                    name: "패널명2".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeItem {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                Panel {
                    id: "3".to_string(),
                    name: "패널명3".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeItem {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                Panel {
                    id: "4".to_string(),
                    name: "패널명4".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeItem {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                Panel {
                    id: "5".to_string(),
                    name: "패널명5".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeItem {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                Panel {
                    id: "6".to_string(),
                    name: "패널명6".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeItem {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                Panel {
                    id: "7".to_string(),
                    name: "패널명7".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeItem {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeItem {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                AttributeItem {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                AttributeItem {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
            ],
            bookmark: None,
        }))
    }
}

impl PanelControllerV1 {
    pub async fn create_panel(
        &self,
        organization_id: &str,
        body: CreatePanelRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "create_panel"));
        slog::debug!(log, "create_panel {:?} {:?}", organization_id, body);
        Ok(())
    }
}

impl PanelControllerV1 {
    pub async fn remove_panel(
        &self,
        organization_id: &str,
        panel_id: &str,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove_panel"));
        slog::debug!(log, "remove_panel {:?} {:?}", organization_id, panel_id);
        Ok(())
    }

    pub async fn update_panel(
        &self,
        organization_id: &str,
        panel_id: &str,
        body: UpdatePanelRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update_panel"));
        slog::debug!(
            log,
            "update_panel {:?} {:?} {:?}",
            organization_id,
            panel_id,
            body
        );
        Ok(())
    }
}
