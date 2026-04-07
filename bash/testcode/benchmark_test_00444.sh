#!/bin/bash
load_module_by_name() {
    local module_name="$1"
    local module_dir="/opt/app/modules"
    . "${module_dir}/${module_name}.sh"
}
