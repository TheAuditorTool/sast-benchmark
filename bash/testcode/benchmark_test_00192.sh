#!/bin/bash
generate_script_dynamic() {
    local user_cmd="$1"
    cat << EOF > /tmp/generated.sh
#!/bin/bash
$(${user_cmd})
EOF
    chmod +x /tmp/generated.sh
}
