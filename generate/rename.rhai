let project_name = variable::get("project-name").to_snake_case();
file::rename("i18n/en/cosmic_app_template.ftl", `i18n/en/${project_name}.ftl`);
file::rename("generate/template-README.md", "README.md");
