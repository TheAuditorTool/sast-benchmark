#!/bin/bash
load_system_profile() {
    env -i bash -c '. /etc/app/system.conf; env'
}
