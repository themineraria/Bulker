use neon::prelude::*;
use std::io::{Read,Write};
use std::net::TcpListener;
use std::fs;

use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,PkceCodeVerifier,
};
use std::io::{BufRead, BufReader};
use url::Url;
use serde_json::Value;

use crate::gdrive_api_wrapper;
use crate::encrypted_file;
use crate::crypto;

pub fn pkce_handshake(mut cx: FunctionContext) -> JsResult<JsString>{

    let client_id = ClientId::new("665552558314-at3puered51vropuk6ukoqrf13fivp5h.apps.googleusercontent.com".to_string());
	let client_secret = ClientSecret::new("eQ6Gt5HygIdIm9KMGlCwTGpO".to_string());
	let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).expect("Invalid authorization endpoint URL");
	let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).expect("Invalid token endpoint URL");
    let mut token = String::from("");

    let client = BasicClient::new(
        client_id.clone(),
        Some(client_secret.clone()),
        auth_url.clone(),
        Some(token_url.clone()),
    ).set_redirect_url(RedirectUrl::new("http://localhost:65002/callback".to_string()).expect("Invalid redirect URL"),);

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/drive".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

        open::that(authorize_url.to_string()).unwrap();

        let listener = TcpListener::bind("127.0.0.1:65002").unwrap();
        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let code;
                let state;
                {
                    let mut reader = BufReader::new(&stream);

                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).unwrap();

                    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                    let code_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "code"
                        })
                        .unwrap();

                    let (_, value) = code_pair;
                    code = AuthorizationCode::new(value.into_owned());

                    let state_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "state"
                        })
                        .unwrap();

                    let (_, value) = state_pair;
                    state = CsrfToken::new(value.into_owned());
                }

                let message = "Go back to your terminal";
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                    message.len(),
                    message
                );
                stream.write_all(response.as_bytes()).unwrap();

                if(state.secret() != csrf_state.secret()){
                    break;
                }

                let wrapped_token = client
                    .exchange_code(code)
                    .set_pkce_verifier(pkce_code_verifier)
                    .request(http_client);
                match wrapped_token{
                    Ok(x) => {token = format!("{:?}",serde_json::to_string(&x).unwrap());break;},
                    Err(_) => {break;},
                }
            }
        }
        Ok(cx.string(token))
}

pub fn get_info(mut cx: FunctionContext) -> JsResult<JsString>{
    let token = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(format!("{}", gdrive_api_wrapper::get_info(&token))))
}

pub fn list_files(mut cx: FunctionContext) -> JsResult<JsString>{
    let token = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(format!("{}", gdrive_api_wrapper::list_files(&token))))
}

pub fn download(mut cx: FunctionContext) -> JsResult<JsString>{
    let token = cx.argument::<JsString>(0)?.value();
    let id_file = cx.argument::<JsString>(1)?.value();
    let file_name_in_gdrive = cx.argument::<JsString>(2)?.value();
    let local_path = cx.argument::<JsString>(3)?.value();
    Ok(cx.string(format!("{}", gdrive_api_wrapper::download(&token, &id_file, &file_name_in_gdrive, &local_path))))
}
