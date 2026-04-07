#!/bin/bash
validate_playbook() {
    ansible-playbook --check -i hosts site.yml
}
