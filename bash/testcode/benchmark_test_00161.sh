#!/bin/bash
generate_template() {
    cat << 'EOF' > /tmp/template.sh
#!/bin/bash
echo "This is a static template"
echo "No variable expansion happens here"
EOF
    chmod +x /tmp/template.sh
}
