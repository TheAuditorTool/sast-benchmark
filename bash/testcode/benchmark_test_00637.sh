#!/bin/bash
archive_user_content() {
    local user_src="$1"
    zip -r /tmp/user_archive.zip "$user_src"
}
