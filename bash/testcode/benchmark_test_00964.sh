#!/bin/bash
get_template() {
    local template_name="$1"
    local lang="$2"
    cat "/opt/templates/${lang}/${template_name}.html"
}
