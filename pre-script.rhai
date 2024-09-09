let template_type = variable::get("template_type");

if template_type == "node" {
    variable::set("package_format", 2);
} else if template_type == "msgs" {
    variable::set("package_format", 3);
}