/// The SDK temporarily supports only one model encoding; others will be added later.
/// Note that this refers to model encoding, not the model itself; one model encoding can apply to multiple models.

pub(crate) mod glm4;

#[derive(Clone)]
pub enum GlmModel {
    /// model encoding：glm-4， can apply to glm-4-0520、glm-4 、glm-4-air、glm-4-airx、 glm-4-flash
    Glm4,
}

impl GlmModel {
    pub(crate) fn from_string(model: String) -> GlmModel {
        match model.as_str() {
            "glm-4" => GlmModel::Glm4,
            _ => panic!("Invalid model"),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        match self {
            GlmModel::Glm4 => "glm-4".to_string(),
        }
    }
}
