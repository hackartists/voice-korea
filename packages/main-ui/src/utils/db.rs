use static_str_ops::staticize;

pub fn cli_for_api(method: &'static str, api: &str) -> easy_dynamodb::Client {
    let api = staticize(api);
    easy_dynamodb::get_client(&crate::utils::logger::new(
        slog::o!("API Method" => method, "API Endpoint" => api),
    ))
}

pub fn get(logger: &slog::Logger) -> easy_dynamodb::Client {
    easy_dynamodb::get_client(&logger)
}
