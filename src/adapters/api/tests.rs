use std::{error::Error, io::Read};

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
    file.read_to_string(&mut raw_schema)
        .expect("Failed to read file!");
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

    let domain_schema = get_raw_schema("domains_by_id_get.json");
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

#[test]
fn check_functions() -> Result<(), Box<dyn Error>> {
    use super::functions;
    use tokio;

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let handle = rt.spawn(functions::get_domains());
    let domains = rt.block_on(handle)??;


    let email = "rusty_tm_6@".to_string() + &domains[0].to_string();
    let password = "32819032189031283".to_string();


    let handle = rt.spawn(functions::create_account(email.clone(), password.clone()));
    let account = rt.block_on(handle)??;



    let handle = rt.spawn(functions::get_token(email.clone(), password.clone()));
    let token = rt.block_on(handle)??;



    let handle = rt.spawn(functions::get_user_from_token(token.token.clone()));
    let retrieved_account = rt.block_on(handle)??;



    assert!(account.id == retrieved_account.id, "Accounts missmatch");
    
    let handle = rt.spawn(functions::get_messages(token.token.clone()));
    rt.block_on(handle)??;

    

    let handle = rt.spawn(functions::delete_account(account.id, token.token.clone()));
    rt.block_on(handle)??;


    Ok(())
}
