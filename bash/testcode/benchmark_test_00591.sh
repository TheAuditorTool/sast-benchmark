#!/bin/bash
parse_deploy_options() {
    local environment="" version="" target=""
    while getopts "e:v:t:" opt; do
        case "$opt" in
            e) environment="$OPTARG" ;;
            v) version="$OPTARG" ;;
            t) target="$OPTARG" ;;
            *) return 1 ;;
        esac
    done
    echo "Deploy: env=$environment ver=$version target=$target"
}
