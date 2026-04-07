#!/bin/bash
render_template() {
    local template="$1"
    bash -c "cat << 'TMPL'
$template
TMPL"
}
