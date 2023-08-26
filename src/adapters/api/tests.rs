use std::io::Read;

const SCHEMAS_FOLDER: &'static str = "./tests/schemas/";


fn get_raw_schema(name: &str) -> String {
    use std::fs::OpenOptions;
    
    let mut filepath = SCHEMAS_FOLDER.to_owned();
    filepath.push_str(name);

    let mut file = OpenOptions::new()
        .read(true)
        .open(&filepath)
        .expect(&format!("Raw schema file not found! {filepath}"));
    
    let mut raw_schema = String::default();
    file.read_to_string(&mut raw_schema).expect("Failed to read file!");
    raw_schema
}

/// This test will check integrity of schemas.
/// 
/// Please not that if in future you are going to add new schema
/// then you'll probably need to add tests for it here
/// 
/// And dont forget to add your new schema to the schemas if needed!

#[test]
fn check_integrity_of_schemas() -> Result<(), serde_json::Error> {
    
    // The test is pretty straightforward, just get the raw schema and try to encode it!
    // If any of errors will occur, then we'll just fail

    let token_schema = get_raw_schema("token.json");
    let _token: super::schemas::Token = serde_json::from_str(&token_schema)?;

    let domain_schema = get_raw_schema( "domains_by_id_get.json");
    let _domain: super::schemas::Domain = serde_json::from_str(&domain_schema)?;

    let domains_schema = get_raw_schema("domains_get.json");
    let _domains: super::schemas::ListOfDomains = serde_json::from_str(&domains_schema)?;

    let account_schema = get_raw_schema("account_get_by_id.json");
    let _account: super::schemas::Account = serde_json::from_str(&account_schema)?;

    let messages_schema = get_raw_schema("messages_get.json");
    let _msgs: super::schemas::ListOfMessages = serde_json::from_str(&messages_schema)?;

    let message_schema = get_raw_schema("messages_get_by_id.json");
    let _msg: super::schemas::Message = serde_json::from_str(&message_schema)?;

    Ok(())
}