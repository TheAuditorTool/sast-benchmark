#!/bin/bash
checkout_repository() {
    local host="$1"
    svn checkout "svn://${host}/project/trunk" ./project
}
