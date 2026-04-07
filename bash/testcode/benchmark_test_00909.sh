#!/bin/bash
load_module() {
    local module_name="$1"
    eval "import_${module_name}"
}
