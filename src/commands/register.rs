use std::io;
use std::io::prelude::*;

use super::super::config::Config;
use super::super::helpers::path::build_path;
use super::super::helpers::oauth2;
use super::super::helpers::providers;
use super::super::results::{BearerResult, BearerError};

fn read_stdin(message: &str) -> BearerResult<String> {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => Ok(buffer.trim().to_string()),
        Err(err) => Err(BearerError::IOError(format!("{}", err))),
    }
}


pub fn command(config_dir: &str, client_name: &str) -> BearerResult<()> {

    debug!("Registering new client {} in directory {}",
           client_name,
           config_dir);
    let (_, exists) = build_path(config_dir, client_name)?;
    if exists {
        return Err(BearerError::ValueError(format!("Client {} already registered", client_name)));
    }
    println!("Before continue, register the a client with the following url to the OAuth2 \
              Provider:");
    println!("");
    println!("http://localhost:6750/callback");
    println!("");
    println!("Ensure your port is not already open by another service.");
    println!("If the provider require a https url, please run an https reverse proxy before \
              continue.");
    println!("");
    let mut provider_name = read_stdin("Enter the OAuth2.0 Provider Name: ")?;

    let (authorize_url, token_url) = match providers::get_provider(&provider_name) {
        Some(provider) => {
            provider_name = provider.name.to_string();
            (provider.authorize_url.to_string(), provider.token_url.to_string())
        }
        None => {
            let authorize_url = read_stdin("Enter the OAuth2.0 Authorize Url: ")?;
            let token_url = read_stdin("Enter the OAuth2.0 Token Url: ")?;
            (authorize_url, token_url)
        }
    };
    let client_id = read_stdin("Enter the Client Id: ")?;
    let secret = read_stdin("Enter the Client Secret: ")?;
    let scope = read_stdin("Enter the scope (optional): ")?;

    let mut conf = Config::new(config_dir,
                               client_name,
                               provider_name.as_str(),
                               authorize_url.as_str(),
                               token_url.as_str(),
                               client_id.as_str(),
                               secret.as_str(),
                               match scope.len() {
                                   0 => None,
                                   _ => Some(scope.as_str()),
                               })?;

    println!("");
    println!("Visit to finish the configuration: http://localhost:6750/callback");

    debug!("Start server to retrieve tokens");
    let tokens = oauth2::get_tokens(&conf, 6750)?;
    debug!("Token retrieved: {:?}", tokens);
    conf.set_tokens(tokens);
    conf.write()?;
    println!("Tokens retrieved succesfully");
    Ok(())
}

