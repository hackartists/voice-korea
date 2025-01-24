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
    // common::CommonQueryResponse, 
    middleware::auth::authorization_middleware,
};

use models::prelude::*;

#[derive(Clone, Debug)]
pub struct AttributeControllerV1 {
    log: slog::Logger,
}

impl AttributeControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "AttributeControllerV1"));
        let ctrl = AttributeControllerV1 { log };

        Router::new()
            .route("/", post(Self::act_attribute).get(Self::list_attributes))
            .route(
                "/:attribute_id",
                post(Self::act_attribute_by_id).get(Self::get_attribute),
            )
            .with_state(ctrl)
            .layer(middleware::from_fn(authorization_middleware))
    }

    pub async fn act_attribute(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<AttributeControllerV1>,
        Json(body): Json<AttributeActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_attribute"));
        slog::debug!(
            log,
            "act_attribute: {:?} {:?}",
            organization_id,
            body.clone()
        );

        match body {
            AttributeActionRequest::Create(req) => {
                ctrl.create_attribute(&organization_id, req).await?;
            }
        }
        Ok(())
    }

    pub async fn act_attribute_by_id(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<AttributeControllerV1>,
        Path(attribute_id): Path<String>,
        Json(body): Json<AttributeByIdActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_attribute_by_id"));
        slog::debug!(
            log,
            "act_attribute_by_id: {:?} {:?}",
            organization_id,
            attribute_id
        );

        match body {
            AttributeByIdActionRequest::Delete => {
                ctrl.remove_attribute(&organization_id, &attribute_id)
                    .await?;
            }
            AttributeByIdActionRequest::Update(req) => {
                ctrl.update_attribute(&organization_id, &attribute_id, req)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn get_attribute(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<AttributeControllerV1>,
        Path(attribute_id): Path<String>,
    ) -> Result<Json<()>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "get_attribute"));
        slog::debug!(
            log,
            "get_attribute: {:?} {:?}",
            organization_id,
            attribute_id
        );

        Ok(Json(()))
    }

    pub async fn list_attributes(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<AttributeControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "list_attributes"));
        slog::debug!(
            log,
            "list_attributes {:?} {:?}",
            organization_id,
            pagination
        );

        // Ok(Json(CommonQueryResponse {
        //     items: vec![

        //     ],
        //     bookmark: None,
        // }))
        Ok(())
    }
}

impl AttributeControllerV1 {
    pub async fn create_attribute(
        &self,
        organization_id: &str,
        body: CreateAttributeRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "create_attribute"));
        slog::debug!(log, "create_attribute {:?} {:?}", organization_id, body);
        let cli = easy_dynamodb::get_client(&log);

        let attribute = Attribute::new(
            organization_id.to_string(),
            body.name,
        );

        let _ = cli
            .upsert(&attribute)
            .await
            .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

        for item in body.attribute_item {
            let attribute_item = AttributeItem::new(
                attribute.id.clone(),
                item.name,
            );

            let _ = cli
                .upsert(&attribute_item)
                .await
                .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;
        }

        Ok(())
    }
}

impl AttributeControllerV1 {
    pub async fn remove_attribute(
        &self,
        organization_id: &str,
        attribute_id: &str,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove_attribute"));
        slog::debug!(
            log,
            "remove_attribute {:?} {:?}",
            organization_id,
            attribute_id
        );
        Ok(())
    }

    pub async fn update_attribute(
        &self,
        organization_id: &str,
        attribute_id: &str,
        body: UpdateAttributeRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update_attribute"));
        slog::debug!(
            log,
            "update_attribute {:?} {:?} {:?}",
            organization_id,
            attribute_id,
            body
        );
        Ok(())
    }
}
