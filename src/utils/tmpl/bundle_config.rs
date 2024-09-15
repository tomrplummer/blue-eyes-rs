use crate::dirs::Dir;
use crate::writable_template::WritableTemplate;

pub struct BundleConfig {
    pub path: String,
    pub template_path: String,
}

impl WritableTemplate for BundleConfig {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn template_path(&self) -> String {
        self.template_path.to_string()
    }
}

impl BundleConfig {
    pub fn new() -> BundleConfig {
        BundleConfig {
            path: Dir::BundleConfig(Some("config")).path(),
            template_path: "bundle_config.template".to_string(),
        }
    }
}