#!/bin/bash
create_shared_group_dir() {
    umask 002
    mkdir /var/app/team_shared
}
