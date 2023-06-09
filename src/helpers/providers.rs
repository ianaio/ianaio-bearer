use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Provider {
    pub name: &'static str,
    pub authorize_url: &'static str,
    pub token_url: &'static str,
}

type Providers = HashMap<&'static str, Provider>;


fn known_providers() -> Providers {
    let mut providers: Providers = HashMap::with_capacity(3);
    providers.insert("gandi",
                     Provider {
                         name: "Gandi",
                         authorize_url: "https://id.gandi.net/authorize",
                         token_url: "https://id.gandi.net/token",
                     });
    providers.insert("github",
                     Provider {
                         name: "Github",
                         authorize_url: "https://github.com/login/oauth/authorize",
                         token_url: "https://github.com/login/oauth/access_token",
                     });
    providers.insert("google",
                     Provider {
                         name: "Google",
                         authorize_url: "https://accounts.google.com/o/oauth2/v2/auth",
                         token_url: "https://www.googleapis.com/oauth2/v4/token",
                     });
    providers
}


pub fn get_provider(name: &str) -> Option<Provider> {
    let name = name.to_lowercase();
    match known_providers().get(name.as_str()) {
        Some(provider) => Some(provider.clone()),
        None => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_provier_known_provider() {
        let provider = get_provider("GANDI");
        assert_eq!(provider.is_some(), true);
        let provider = provider.unwrap();
        assert_eq!(provider.name, "Gandi");
    }

    #[test]
    fn test_get_provier_unknown_provider() {
        let provider = get_provider("NXPROVIDER");
        assert_eq!(provider.is_none(), true);
    }

}

