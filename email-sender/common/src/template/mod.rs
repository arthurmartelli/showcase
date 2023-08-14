use crate::data::DataMap;
use handlebars::{Handlebars, RenderError};
use std::fs::{self, ReadDir};

#[derive(Debug)]
pub struct Template {
    template_name: String,
    templates_dir: String,
    layout_dir: String,
    partials_dir: String,
    file_extension: String,
    views_dir: String,
}

impl Template {
    pub fn new<T: Into<String>>(template: T) -> Self {
        Self {
            template_name: template.into(),
            templates_dir: "templates".to_string(),
            layout_dir: "layouts".to_string(),
            partials_dir: "partials".to_string(),
            file_extension: "hbs".to_string(),
            views_dir: "./views/".to_string(),
        }
    }

    pub fn get_plain(&self) -> String {
        String::from("")
    }

    pub fn get_html(&self, context: &DataMap) -> Result<String, RenderError> {
        let hbs: Handlebars = self.load_template()?;

        hbs.render("main_layout", &context)
    }

    pub fn load_template(&self) -> Result<Handlebars, RenderError> {
        let mut hbs: Handlebars = Handlebars::new();

        self.register_dirs(&mut hbs)?;
        self.register_template(&mut hbs)?;

        Ok(hbs)
    }

    fn register_template(&self, hbs: &mut Handlebars) -> Result<(), RenderError> {
        let name: &str = "main";
        let path: String = format!(
            "{}/{}/{}.{}",
            &self.views_dir, &self.templates_dir, &self.template_name, &self.file_extension
        );

        hbs.register_template_file(name, path)?;

        Ok(())
    }

    fn register_dirs(&self, hbs: &mut Handlebars) -> Result<(), RenderError> {
        let paths: Vec<&String> = vec![&self.layout_dir, &self.partials_dir];

        for path in paths {
            let dir: String = format!("{}/{}/", &self.views_dir, path);
            self.register_path(hbs, dir)?;
        }

        Ok(())
    }

    fn register_path(&self, hbs: &mut Handlebars, dir: String) -> Result<(), RenderError> {
        let paths: ReadDir = fs::read_dir(dir)?;

        for path in paths {
            let file_path = path.as_ref().unwrap().path();
            let file_name = path.as_ref().unwrap().file_name();

            if let Some(name) = file_name.to_str() {
                let file_name_separated: Vec<&str> = name.split_terminator('.').collect();
                hbs.register_template_file(file_name_separated[0], file_path)?
            };
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn set_views_dir(&mut self, views_dir: String) {
        self.views_dir = views_dir;
    }

    #[allow(unused)]
    pub fn set_templates_dir(&mut self, templates_dir: String) {
        self.templates_dir = templates_dir;
    }

    #[allow(unused)]
    pub fn set_layout_dir(&mut self, layout_dir: String) {
        self.layout_dir = layout_dir;
    }

    #[allow(unused)]
    pub fn set_partials_dir(&mut self, partials_dir: String) {
        self.partials_dir = partials_dir;
    }

    #[allow(unused)]
    pub fn set_file_extension(&mut self, file_extension: String) {
        self.file_extension = file_extension;
    }
}
