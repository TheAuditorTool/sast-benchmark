#!/bin/bash
checkout_repo_encrypted() {
    local host="$1"
    svn checkout "svn+ssh://${host}/project/trunk" ./project
}
