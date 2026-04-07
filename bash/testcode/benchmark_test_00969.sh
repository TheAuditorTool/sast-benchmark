#!/bin/bash
get_locale_file() {
    local lang="$1"
    local cleaned
    cleaned=$(echo "$lang" | tr -dc 'a-zA-Z_')
    cat "/usr/share/app/locales/${cleaned}.po"
}
