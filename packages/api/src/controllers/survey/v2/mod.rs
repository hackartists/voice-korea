use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::post,
        Extension, Json,
    },
};
use models::*;
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct SurveyControllerV2 {
    panel_survey_repo: PanelSurveysRepository,
    repo: SurveyV2Repository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SurveyControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = SurveyV2::get_repository(pool.clone());
        let panel_survey_repo = PanelSurveys::get_repository(pool.clone());
        let ctrl = SurveyControllerV2 {
            repo,
            panel_survey_repo,
            pool,
        };

        Ok(by_axum::axum::Router::new()
            .route("/:id", post(Self::act_by_id).get(Self::get_survey_v2))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_survey_v2).get(Self::list_survey_v2))
            .with_state(ctrl.clone()))
    }

    pub async fn act_by_id(
        State(ctrl): State<SurveyControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((org_id, id)): Path<(i64, i64)>,
        Json(body): Json<SurveyV2ByIdAction>,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("act_by_id: {:?} {:?}", id, body);

        match body {
            SurveyV2ByIdAction::Update(params) => ctrl.update(org_id, id, params).await,
        }
    }

    pub async fn act_survey_v2(
        State(ctrl): State<SurveyControllerV2>,
        Path(org_id): Path<i64>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<SurveyV2Action>,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("act_survey_v2 {:?}", body);
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        match body {
            SurveyV2Action::Create(body) => ctrl.create(org_id, body).await,
            SurveyV2Action::Delete(body) => ctrl.delete(body.id).await,
        }
    }

    pub async fn get_survey_v2(
        State(ctrl): State<SurveyControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((org_id, id)): Path<(i64, i64)>,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("get_survey_v2 {:?}", id);
        let survey = ctrl
            .repo
            .find_one(&SurveyV2ReadAction::new().find_by_id(id))
            .await?;

        if survey.org_id != org_id {
            return Err(ApiError::Unauthorized);
        }

        Ok(Json(survey))
    }

    pub async fn list_survey_v2(
        State(ctrl): State<SurveyControllerV2>,
        Path(org_id): Path<i64>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<SurveyV2Param>,
    ) -> Result<Json<SurveyV2GetResponse>> {
        tracing::debug!("list_survey_v2 {:?}", q);

        match q {
            SurveyV2Param::Query(q) => ctrl.find(org_id, q).await,
            _ => Err(ApiError::InvalidAction),
        }
    }
}

impl SurveyControllerV2 {
    pub async fn find(
        &self,
        org_id: i64,
        SurveyV2Query { size, bookmark, .. }: SurveyV2Query,
    ) -> Result<Json<SurveyV2GetResponse>> {
        let mut total_count: i64 = 0;

        // let query = SurveyV2Summary::base_sql_with("where org_id = $1 limit $2 offset $3");

        // FIXME: fix to this line bug
        // query.push_str(" order by id desc");
        // tracing::debug!("find query: {}", query);

        let items: Vec<SurveyV2Summary> = sqlx::query(
            &"SELECT 
        COUNT(*) OVER() AS total_count, 
        p.id, 
        p.created_at, 
        p.updated_at, 
        p.name, 
        p.project_type, 
        p.project_area, 
        p.status, 
        p.started_at, 
        p.ended_at, 
        p.quotes, 
        p.org_id, 
        p.panel_counts, 
        COALESCE(
            json_agg(to_jsonb(panels)) FILTER (WHERE panels.id IS NOT NULL), '[]'
        ) AS panels
    FROM surveys p 
    LEFT JOIN panel_surveys ps ON p.id = ps.survey_id
    LEFT JOIN panels panels ON ps.panel_id = panels.id
    WHERE p.org_id = $1 
    GROUP BY p.id, p.created_at, p.updated_at, p.name, p.project_type, 
             p.project_area, p.status, p.started_at, p.ended_at, p.quotes, 
             p.org_id, p.panel_counts
    LIMIT $2 OFFSET $3;",
        )
        .bind(org_id)
        .bind(size as i64)
        .bind(size as i64 * (bookmark.unwrap_or("1".to_string()).parse::<i64>().unwrap() - 1))
        .map(|r: PgRow| {
            use sqlx::Row;

            total_count = r.get("total_count");
            r.into()
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(Json(SurveyV2GetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }

    pub async fn delete(&self, id: i64) -> Result<Json<SurveyV2>> {
        //FIXME: receive panel params and remove panel data
        tracing::debug!("delete survey: {:?}", id);

        let _ = self.repo.delete(id).await?;

        Ok(Json(SurveyV2::default()))
    }

    pub async fn update(
        &self,
        org_id: i64,
        id: i64,
        body: SurveyV2UpdateRequest,
    ) -> Result<Json<SurveyV2>> {
        //FIXME: receive panel params and update panel data
        let survey = self
            .repo
            .update(
                id,
                SurveyV2RepositoryUpdateRequest {
                    name: Some(body.name),
                    project_type: Some(body.project_type),
                    project_area: Some(body.project_area),
                    status: Some(body.status),
                    started_at: Some(body.started_at),
                    ended_at: Some(body.ended_at),
                    description: Some(body.description),
                    quotes: Some(body.quotes),
                    org_id: Some(org_id),
                    questions: Some(body.questions),
                    panel_counts: Some(body.panel_counts),
                },
            )
            .await?;
        Ok(Json(survey))
    }

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
                body.panel_counts.clone(),
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
