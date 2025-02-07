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
    panel_survey_repo: PanelSurveysRepository,
    repo: SurveyV2Repository,
}

impl SurveyControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = SurveyV2::get_repository(pool.clone());
        let panel_survey_repo = PanelSurveys::get_repository(pool);

        let ctrl = SurveyControllerV2 {
            repo,
            panel_survey_repo,
        };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_survey_v2))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_survey_v2).get(Self::list_survey_v2))
            .with_state(ctrl.clone()))
    }

    pub async fn act_survey_v2(
        State(ctrl): State<SurveyControllerV2>,
        Path(org_id): Path<String>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<SurveyV2Action>,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("act_survey_v2 {:?}", body);
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        match body {
            SurveyV2Action::Create(body) => ctrl.create(org_id.parse::<i64>().unwrap(), body).await,
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
        Path((org_id, id)): Path<(String, String)>,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("get_survey_v2 {:?}", id);
        let survey = ctrl
            .repo
            .find_one(&SurveyV2ReadAction::new().find_by_id(id.parse::<i64>().unwrap()))
            .await?;

        if survey.org_id != org_id.parse::<i64>().unwrap() {
            return Err(ApiError::Unauthorized);
        }

        Ok(Json(survey))
    }

    pub async fn list_survey_v2(
        State(ctrl): State<SurveyControllerV2>,
        Path(org_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<SurveyV2Param>,
    ) -> Result<Json<SurveyV2GetResponse>> {
        tracing::debug!("list_survey_v2 {:?}", q);

        match q {
            SurveyV2Param::Query(q) => Ok(Json(SurveyV2GetResponse::Query(
                ctrl.repo
                    .find(&q.with_org_id(org_id.parse::<i64>().unwrap()))
                    .await?,
            ))),
            _ => Err(ApiError::InvalidAction),
        }
    }
}

impl SurveyControllerV2 {
    pub async fn create(&self, org_id: i64, body: SurveyV2CreateRequest) -> Result<Json<SurveyV2>> {
        tracing::debug!("create {:?} {:?}", org_id, body);

        let survey = self
            .repo
            .insert(
                body.name.clone(),
                ProjectType::Survey,
                body.project_area,
                ProjectStatus::Ready,
                body.started_at,
                body.ended_at,
                body.description.clone(),
                body.quotes,
                org_id.clone(),
                body.questions.clone(),
            )
            .await?;

        for panel in body.panels.clone() {
            let _ = self
                .panel_survey_repo
                .insert(panel.id.clone(), survey.id.clone())
                .await?;
        }

        Ok(Json(survey))
    }
}
