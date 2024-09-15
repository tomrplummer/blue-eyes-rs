use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Dir<'a> {
    App(Option<&'a str>),
    Controllers(Option<&'a str>),
    Models(Option<&'a str>),
    Styles(Option<&'a str>),
    Views(Option<&'a str>),
    Bin(Option<&'a str>),
    Db(Option<&'a str>),
    Migrations(Option<&'a str>),
    Public(Option<&'a str>),
    Stylesheets(Option<&'a str>),
    PathsPlugins(Option<&'a str>),
    BundleConfig(Option<&'a str>),
    Helpers(Option<&'a str>),
    Root(Option<&'a str>),
}

impl<'a> Dir<'a> {
    fn create_path(mut parts: Vec<&'a str>, filename: Option<&'a str>) -> PathBuf {
        parts.insert(0, ".");

        match filename {
            Some(f) => {
                parts.push(f);
                parts.iter().collect()
            }
            None => parts.iter().collect(),
        }
    }

    pub fn path(&self) -> String {
        let joined_path = match self {
            Dir::Root(filename) => Dir::create_path(vec![], *filename),
            Dir::App(filename) => Dir::create_path(vec!["app"], *filename),
            Dir::Controllers(filename) => Dir::create_path(vec!["app", "controllers"], *filename),
            Dir::Models(filename) => Dir::create_path(vec!["app", "models"], *filename),
            Dir::Styles(filename) => Dir::create_path(vec!["app", "styles"], *filename),
            Dir::Views(filename) => Dir::create_path(vec!["app", "views"], *filename),
            Dir::Bin(filename) => Dir::create_path(vec!["bin"], *filename),
            Dir::Db(filename) => Dir::create_path(vec!["db"], *filename),
            Dir::Migrations(filename) => Dir::create_path(vec!["db", "migrations"], *filename),
            Dir::Public(filename) => Dir::create_path(vec!["public"], *filename),
            Dir::Stylesheets(filename) => {
                Dir::create_path(vec!["public", "stylesheets"], *filename)
            }
            Dir::PathsPlugins(filename) => Dir::create_path(vec!["plugins", "paths"], *filename),
            Dir::BundleConfig(filename) => Dir::create_path(vec![".bundle"], *filename),
            Dir::Helpers(filename) => Dir::create_path(vec!["helpers"], *filename),
        };

        joined_path.to_str().unwrap().to_string()
    }
}
