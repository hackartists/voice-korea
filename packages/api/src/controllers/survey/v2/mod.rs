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
pub struct SurveyControllerV2 {
    repo: SurveyV2Repository,
    user: UserRepository,
}

impl SurveyControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let user = User::get_repository(pool.clone());
        let repo = SurveyV2::get_repository(pool);

        let ctrl = SurveyControllerV2 { repo, user };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_survey_v2))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_survey_v2).get(Self::list_survey_v2))
            .with_state(ctrl.clone()))
    }

    pub async fn act_survey_v2(
        State(ctrl): State<SurveyControllerV2>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<SurveyV2Action>,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("act_survey_v2 {:?}", body);
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        match body {
            SurveyV2Action::Create(body) => ctrl.create(auth.unwrap(), body).await,
        }
    }

    // pub async fn act_survey_v2_by_id(
    //     State(_ctrl): State<SurveyControllerV2>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(id): Path<String>,
    //     Json(body): Json<SurveyV2ByIdAction>,
    // ) -> Result<Json<SurveyV2>> {
    //     tracing::debug!("act_survey_v2_by_id {:?} {:?}", id, body);
    //     Ok(Json(SurveyV2::default()))
    // }

    pub async fn get_survey_v2(
        State(ctrl): State<SurveyControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("get_survey_v2 {:?}", id);
        let survey = ctrl
            .repo
            .find_one(&SurveyV2ReadAction::new().find_by_id(id))
            .await?;

        Ok(Json(survey))
    }

    pub async fn list_survey_v2(
        State(_ctrl): State<SurveyControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<SurveyV2Param>,
    ) -> Result<Json<SurveyV2GetResponse>> {
        tracing::debug!("list_survey_v2 {:?}", q);

        match q {
            SurveyV2Param::Query(q) => {
                Ok(Json(SurveyV2GetResponse::Query(_ctrl.repo.find(&q).await?)))
            }
            _ => Err(ApiError::InvalidAction),
        }
    }
}

impl SurveyControllerV2 {
    pub async fn has_permission(&self, auth: Authorization, org_id: String) -> Result<bool> {
        match auth {
            Authorization::Bearer { claims } => {
                let user_id = claims.sub;
                // FIXME: optimize permission check by getting data by user_id and org_id
                let user = self
                    .user
                    .find_one(&UserReadAction::new().find_by_id(user_id))
                    .await?;

                Ok(user.orgs.iter().filter(|org| org.id == org_id).count() > 0)
            }
            _ => Ok(false),
        }
    }

    pub async fn create(
        &self,
        auth: Authorization,
        body: SurveyV2CreateRequest,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("create {:?} {:?}", auth, body);
        if !self.has_permission(auth, body.org_id.clone()).await? {
            return Err(ApiError::Unauthorized);
        }

        let survey = self
            .repo
            .insert(
                body.name,
                ProjectType::Survey,
                body.project_area,
                body.started_at,
                body.ended_at,
                body.description,
                body.quotes,
            )
            .await?;

        Ok(Json(survey))
    }
}
