use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::get,
        Extension, Json, Router,
    },
};

use models::{
    // ResourceDeleteRequest,
    Resource,
    ResourceAction,
    ResourceByIdAction,
    ResourceCreateRequest,
    ResourceGetResponse,
    ResourceParam,
    ResourceReadAction,
    ResourceRepository,
    ResourceUpdateRequest,
};

#[derive(Clone, Debug)]
pub struct ResourceConterollerV1 {
    repo: ResourceRepository,
}

impl ResourceConterollerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> models::Result<Router> {
        let repo = Resource::get_repository(pool.clone());
        let ctrl = Self { repo };

        Ok(Router::new()
            .route(
                "/:id",
                get(Self::get_resource).post(Self::act_resource_by_id),
            )
            .route("/", get(Self::list_resources).post(Self::act_resource))
            .with_state(ctrl))
    }
    async fn get_resource(
        State(ctrl): State<ResourceConterollerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> models::Result<Json<Resource>> {
        let resource = ctrl
            .repo
            .find_one(&ResourceReadAction::new().find_by_id(id))
            .await?;
        Ok(Json(resource))
    }

    async fn list_resources(
        State(ctrl): State<ResourceConterollerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(params): Query<ResourceParam>,
    ) -> models::Result<Json<ResourceGetResponse>> {
        match params {
            ResourceParam::Query(q) => {
                Ok(Json(ResourceGetResponse::Query(ctrl.repo.find(&q).await?)))
            }
            ResourceParam::Read(q) => Ok(Json(ResourceGetResponse::Read(
                ctrl.repo.find_one(&q).await?,
            ))),
        }
    }

    async fn act_resource(
        State(ctrl): State<ResourceConterollerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<ResourceAction>,
    ) -> models::Result<Json<Resource>> {
        match body {
            ResourceAction::Create(req) => {
                let res = ctrl.create(req).await?;
                Ok(Json(res))
            }
        }
    }

    async fn act_resource_by_id(
        State(ctrl): State<ResourceConterollerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<ResourceByIdAction>,
    ) -> models::Result<Json<Resource>> {
        //TODO: Check Permission
        match body {
            ResourceByIdAction::Update(req) => {
                let res = ctrl.update(req).await?;
                Ok(Json(res))
            } // ResourceByIdAction::Delete(reqwest) => {
              //     let res = Self::delete(ctrl.repo.clone(), reqwest.id).await?;
              //     Ok(Json(res))
              // }
        }
    }
}
impl ResourceConterollerV1 {
    async fn create(&self, req: ResourceCreateRequest) -> models::Result<Resource> {
        tracing::debug!("create_resource: {:?}", req);
        let resource = self
            .repo
            .insert(
                req.title,
                req.resource_type,
                req.field,
                req.usage_purpose,
                req.source,
                req.access_level,
                req.org_id,
            )
            .await?;
        Ok(resource)
    }
    async fn update(&self, req: ResourceUpdateRequest) -> models::Result<Resource> {
        tracing::debug!("update_resource: {:?}", req);
        // TODO: Update Resource
        Ok(Resource::default())
    }
    #[allow(unused)]
    async fn delete(&self, id: String) -> models::Result<Resource> {
        tracing::debug!("delete_resource: {:?}", id);
        // TODO: Soft-delete Resource
        Ok(Resource::default())
    }
}
