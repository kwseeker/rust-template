/// Create and store JWT
/// ChatGLM support two kinds of authentication: API Key and JWT.
/// details: https://open.bigmodel.cn/dev/api#http_auth
pub(crate) struct JwtBuilder{
    secret: String,
    header: String,
    payload: String,
}

pub(crate) struct JwtHolder {
    token: Option<String>,
}

pub(crate) static JWT_HOLDER: JwtHolder = JwtHolder {
    token: None,
};

impl JwtHolder {
    pub(crate) fn get_jwt(&self) -> String {
        todo!();
    }
}
