#!/bin/bash
load_profile() {
    local profile_path="$1"
    source "$profile_path"
}
