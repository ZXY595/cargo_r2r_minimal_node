let template_type = variable::get("template_type");

if template_type == "node" {
    variable::set("package_format", "2");
    variable::set("use_template_node", true);
} else if template_type == "msgs" {
    variable::set("package_format", "3");
    variable::set("use_template_msgs", true);
}

