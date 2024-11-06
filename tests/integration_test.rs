use include_remote_str::include_remote_str;

#[tokio::test]
async fn it_fetches() {
    let file = include_remote_str!("https://raw.githubusercontent.com/ua-parser/uap-core/refs/heads/master/regexes.yaml");
    assert!(file.starts_with("user_agent_parsers:"));

}
