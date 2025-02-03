impl Organization
{
    pub fn get_repository(pool : sqlx :: Pool < sqlx :: Postgres >) ->
    OrganizationRepository { OrganizationRepository :: new(pool) }
} impl Organization
{
    pub fn base_sql() -> String
    {
        format!
        ("SELECT p.*, \nCOALESCE(\n    json_agg(\n        jsonb_set(\n            to_jsonb(f),\n            '{{id}}',\n            to_jsonb(f.id::TEXT)\n        )\n    ) FILTER (WHERE f.id IS NOT NULL), '[]'\n) AS users\n FROM organizations p \nLEFT JOIN user_orgs j ON p.id = j.org_id\nLEFT JOIN users f ON j.user_id = f.id\n",)
    } pub fn group_by() -> String { "GROUP BY p.id".to_string() }
} #[derive(Debug, Clone)] pub struct OrganizationRepository
{ pool : sqlx :: Pool < sqlx :: Postgres > , } impl OrganizationRepository
{
    pub fn new(pool : sqlx :: Pool < sqlx :: Postgres >) -> Self
    { Self { pool } } pub fn queries(& self) -> Vec < & 'static str >
    {
        vec!
        ["CREATE TABLE IF NOT EXISTS organizations (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,name TEXT NOT NULL);",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_organizations'\n        AND tgrelid = 'organizations'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_organizations\n        BEFORE INSERT ON organizations\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_organizations'\n        AND tgrelid = 'organizations'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_organizations\n        BEFORE INSERT OR UPDATE ON organizations\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$",
        "\nCREATE TABLE IF NOT EXISTS user_orgs (\n    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,\n    user_id BIGINT NOT NULL,\n    org_id BIGINT NOT NULL,\n    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW()),\n\n    CONSTRAINT fk_organizations_users FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,\n    CONSTRAINT fk_users_organizations FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE\n);\n"]
    } pub async fn create_this_table(& self) -> std :: result :: Result < (),
    sqlx :: Error >
    {
        tracing :: debug!
        ("Create table: {}",
        "CREATE TABLE IF NOT EXISTS organizations (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,name TEXT NOT NULL);");
        sqlx ::
        query("CREATE TABLE IF NOT EXISTS organizations (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,name TEXT NOT NULL);").execute(&
        self.pool).await ? ; Ok(())
    } pub async fn create_related_tables(& self) -> std :: result :: Result <
    (), sqlx :: Error >
    {
        for query in
        ["DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_organizations'\n        AND tgrelid = 'organizations'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_organizations\n        BEFORE INSERT ON organizations\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_organizations'\n        AND tgrelid = 'organizations'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_organizations\n        BEFORE INSERT OR UPDATE ON organizations\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$",
        "\nCREATE TABLE IF NOT EXISTS user_orgs (\n    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,\n    user_id BIGINT NOT NULL,\n    org_id BIGINT NOT NULL,\n    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW()),\n\n    CONSTRAINT fk_organizations_users FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,\n    CONSTRAINT fk_users_organizations FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE\n);\n"]
        {
            tracing :: debug! ("Execute queries: {}", query); sqlx ::
            query(query).execute(& self.pool).await ? ;
        } Ok(())
    } pub async fn create_table(& self) -> std :: result :: Result < (), sqlx
    :: Error >
    {
        sqlx ::
        query("CREATE TABLE IF NOT EXISTS organizations (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,name TEXT NOT NULL);").execute(&
        self.pool).await ? ; for query in
        ["DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_organizations'\n        AND tgrelid = 'organizations'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_organizations\n        BEFORE INSERT ON organizations\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_organizations'\n        AND tgrelid = 'organizations'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_organizations\n        BEFORE INSERT OR UPDATE ON organizations\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$",
        "\nCREATE TABLE IF NOT EXISTS user_orgs (\n    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,\n    user_id BIGINT NOT NULL,\n    org_id BIGINT NOT NULL,\n    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW()),\n\n    CONSTRAINT fk_organizations_users FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,\n    CONSTRAINT fk_users_organizations FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE\n);\n"]
        {
            tracing :: debug! ("Execute queries: {}", query); sqlx ::
            query(query).execute(& self.pool).await ? ;
        } Ok(())
    } pub async fn drop_table(& self) -> std :: result :: Result < (), sqlx ::
    Error >
    {
        sqlx ::
        query("DROP TABLE IF EXISTS organizations;").execute(&
        self.pool).await ? ; Ok(())
    } pub async fn insert(& self, name : String) -> Result < Organization >
    {
        tracing :: debug!
        ("insert query: {}",
        "INSERT INTO organizations (name) VALUES ($1) RETURNING id, created_at, updated_at, name");
        let row = sqlx ::
        query("INSERT INTO organizations (name) VALUES ($1) RETURNING id, created_at, updated_at, name").bind(name).map(|
        row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; Organization
            {
                id : row.get :: < i64, _ > ("id").to_string(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), name :
                row.try_get("name").unwrap_or_default(), users : match
                row.try_get :: < serde_json :: Value, _ > ("users")
                {
                    Ok(v) => serde_json :: from_value(v).unwrap(), _ => vec! []
                }
            }
        }).fetch_one(& self.pool).await ? ; Ok(row)
    } pub async fn
    insert_with_dependency(& self, user_id : i64, name : String) -> Result <
    () >
    {
        use sqlx :: Row; use sqlx :: postgres :: PgRow; let mut tx =
        self.pool.begin().await ? ; let row : PgRow = sqlx ::
        query("INSERT INTO organizations (name) VALUES ($1) RETURNING id").bind(name).fetch_one(&
        mut * tx).await ? ; let id : i64 = row.try_get("id") ? ; sqlx ::
        query("INSERT INTO user_orgs (user_id, org_id) VALUES ($1, $2)").bind(user_id).bind(id).execute(&
        mut * tx).await ? ; tx.commit().await ? ; Ok(())
    } pub async fn find_one(& self, param : & OrganizationReadAction) ->
    Result < Organization >
    {
        let mut query = format! ("{}", Organization :: base_sql());
        query.push_str(" ");
        query.push_str(Organization :: group_by().as_str()); tracing :: debug!
        ("{} query {}: {:?}", "OrganizationRepository::find_one", query,
        param); let mut q = sqlx :: query(& query); let row =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; Organization
            {
                id : row.get :: < i64, _ > ("id").to_string(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), name :
                row.try_get("name").unwrap_or_default(), users : match
                row.try_get :: < serde_json :: Value, _ > ("users")
                {
                    Ok(v) => serde_json :: from_value(v).unwrap(), _ => vec! []
                }
            }
        }).fetch_one(& self.pool).await ? ; Ok(row)
    } pub async fn find(& self, param : & OrganizationQuery) -> Result <
    QueryResponse<OrganizationSummary> >
    {
        let query = format!
        ("WITH data AS ({} {}) SELECT ({}) AS total_count, data.* FROM data;",
        "SELECT * FROM organizations", "LIMIT $1 OFFSET $2",
        "SELECT COUNT(*) FROM organizations"); tracing :: debug!
        ("{} query {}", "OrganizationRepository::find_one", query); let offset
        : i32 = (param.size as i32) * (param.page() - 1); let q = sqlx ::
        query(& query).bind(param.size as i32).bind(offset); let mut total :
        i64 = 0; let rows =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; total = row.get("total_count");
            OrganizationSummary
            {
                id : row.get :: < i64, _ > ("id").to_string(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), name :
                row.try_get("name").unwrap_or_default()
            }
        }).fetch_all(& self.pool).await ? ; Ok((rows, total).into())
    }
} impl From < sqlx :: postgres :: PgRow > for Organization
{
    fn from(row : sqlx :: postgres :: PgRow) -> Self
    {
        use sqlx :: Row; Organization
        {
            id : row.get :: < i64, _ > ("id").to_string(), created_at :
            row.try_get("created_at").unwrap_or_default(), updated_at :
            row.try_get("updated_at").unwrap_or_default(), name :
            row.try_get("name").unwrap_or_default(), users : match row.try_get
            :: < serde_json :: Value, _ > ("users")
            { Ok(v) => serde_json :: from_value(v).unwrap(), _ => vec! [] }
        }
    }
}
#[derive(Debug, Clone, serde :: Deserialize, serde :: Serialize, Default, Eq,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct Organization
{
    pub id : String, pub created_at : i64, pub updated_at : i64, pub name :
    String, #[serde(default)] pub users : Vec < User >
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default, Eq,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo, sqlx :: FromRow))] pub
struct OrganizationSummary
{
    pub id : String, pub created_at : i64, pub updated_at : i64, pub name :
    String,
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default, Eq,
PartialEq, by_macros :: QueryDisplay)] #[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
OrganizationQuery
{
    #[serde(deserialize_with = "parse_size_of_organization_query")] pub size :
    usize, pub bookmark : Option < String > ,
} pub fn parse_size_of_organization_query < 'de, D > (deserializer : D) -> std
:: result :: Result < usize, D :: Error > where D : serde :: Deserializer <
'de > ,
{
    use serde :: Deserialize; let s : Option < String > = Option ::
    deserialize(deserializer) ? ;
    s.unwrap_or_else(|| Default :: default()).parse :: < usize >
    ().map_err(serde :: de :: Error :: custom)
} impl OrganizationQuery
{
    pub fn new(size : usize) -> Self { Self { size, .. Self :: default() } }
    pub fn with_bookmark(mut self, bookmark : String) -> Self
    { self.bookmark = Some(bookmark); self } pub fn
    with_page(mut self, page : usize) -> Self
    { self.bookmark = Some(page.to_string()); self } pub fn page(& self) ->
    i32
    {
        self.bookmark.as_ref().unwrap_or(&
        "1".to_string()).parse().unwrap_or(1)
    }
} impl OrganizationClient {} impl Organization
{
    pub fn get_client(endpoint : & str) -> OrganizationClient
    { OrganizationClient { endpoint : endpoint.to_string() } }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default, Eq,
PartialEq)] pub struct OrganizationClient { pub endpoint : String, } impl
OrganizationClient
{
    pub async fn query(& self, params : OrganizationQuery,) -> crate :: Result
    < QueryResponse<OrganizationSummary> >
    {
        let path = format! ("/auth/v1/organizations",); let endpoint = format!
        ("{}{}", self.endpoint, path); let query = format!
        ("{}?{}", endpoint, OrganizationParam :: Query(params)); rest_api ::
        get(& query).await
    } pub async fn get(& self, id : & str) -> crate :: Result < Organization >
    {
        let path = format! ("/auth/v1/organizations",); let endpoint = format!
        ("{}{}/{}", self.endpoint, path, id); rest_api ::
        get(& endpoint).await
    }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default, Eq,
PartialEq, by_macros :: QueryDisplay)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
OrganizationReadAction {} impl OrganizationReadAction
{ pub fn new() -> Self { Self :: default() } } impl OrganizationClient {}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Eq,
PartialEq, by_macros :: QueryDisplay)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))]
#[serde(tag = "param-type", rename_all = "kebab-case")] pub enum
OrganizationParam { Query(OrganizationQuery), }
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(tag = "param_type")] #[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub enum
OrganizationGetResponse { Query(QueryResponse<OrganizationSummary>), }