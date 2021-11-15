use crate::graph::FieldErrorWithCode;
use jsonwebtokens_cognito::KeySet;
use std::env;

pub async fn verify_token(token: &str) -> Result<Option<String>, FieldErrorWithCode> {
    let pool_id = env::var("COGNITE_USER_POOL_ID").expect("need set cognite pool id");
    let client_id = env::var("COGNITE_CLIENT_ID").expect("need set cognite client id");

    let keyset = KeySet::new("ap-northeast-1", pool_id)?;
    let verifier = keyset
        .new_id_token_verifier(&[&client_id])
        .build()
        .map_err(|_err| FieldErrorWithCode::authenticate_error())?;

    let result = keyset
        .verify(token, &verifier)
        .await
        .map_err(FieldErrorWithCode::from)?;

    Ok(result.get("sub").map(|v| v.to_string()))
}
