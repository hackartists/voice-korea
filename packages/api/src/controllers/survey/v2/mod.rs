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

use crate::utils::nonce_lab::NonceLabClient;

#[derive(Clone, Debug)]
pub struct SurveyControllerV2 {
    panel_survey_repo: PanelSurveysRepository,
    repo: SurveyV2Repository,
    pool: sqlx::Pool<sqlx::Postgres>,
    nonce_lab: NonceLabClient,
}

impl SurveyControllerV2 {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = SurveyV2::get_repository(pool.clone());
        let panel_survey_repo = PanelSurveys::get_repository(pool.clone());

        Self {
            repo,
            panel_survey_repo,
            pool,
            nonce_lab: NonceLabClient::new(),
        }
    }

    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let ctrl = Self::new(pool);

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
            &"WITH data AS (
    SELECT p.*, 
        COALESCE(
            json_agg(to_jsonb(f)) FILTER (WHERE f.id IS NOT NULL), '[]'
        ) AS panels
    FROM surveys p
    LEFT JOIN panel_surveys j ON p.id = j.survey_id
    LEFT JOIN panels f ON j.panel_id = f.id
    WHERE p.org_id = $1
    GROUP BY p.id
    LIMIT $2 OFFSET $3
)
SELECT 
    (SELECT COUNT(*) FROM surveys WHERE org_id = $1) AS total_count, 
    data.*
FROM data;",
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
                },
            )
            .await?;
        Ok(Json(survey))
    }

    pub async fn create(
        &self,
        org_id: i64,
        SurveyV2CreateRequest {
            name,
            project_area,
            started_at,
            ended_at,
            description,
            quotes,
            questions,
            panels,
        }: SurveyV2CreateRequest,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("create {:?}", org_id,);
        let mut tx = self.pool.begin().await?;

        let survey = match self
            .repo
            .insert_with_tx(
                &mut *tx,
                name,
                ProjectType::Survey,
                project_area,
                ProjectStatus::Ready,
                started_at,
                ended_at,
                description,
                quotes,
                org_id.clone(),
                questions,
            )
            .await?
        {
            Some(v) => v,
            None => return Err(ApiError::SurveyAlreadyExists),
        };

        for panel in panels.clone() {
            let _ = self
                .panel_survey_repo
                .insert_with_tx(&mut *tx, panel.id, survey.id)
                .await?;
        }

        tx.commit().await?;

        // FIXME: This is workaround. Fix to use mock when testing
        #[cfg(not(test))]
        self.nonce_lab.create_survey(survey.clone().into()).await?;

        Ok(Json(survey))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::tests::*;

    #[tokio::test]
    async fn test_survey_create() {
        let TestContext { user, now, .. } = setup().await.unwrap();

        let cli = SurveyV2::get_client("http://localhost:3000");
        let org_id = user.orgs[0].id;

        let res = cli
            .create(
                org_id,
                "test".to_string(),
                ProjectArea::City,
                now,
                now + 3600,
                "test description".to_string(),
                100,
                vec![],
                vec![],
            )
            .await;

        assert!(res.is_ok(), "{:?}", res);
    }
}
