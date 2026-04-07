#!/bin/bash
stage_user_content() {
    local user_src="$1"
    cp -r "$user_src" /tmp/staging/
}
