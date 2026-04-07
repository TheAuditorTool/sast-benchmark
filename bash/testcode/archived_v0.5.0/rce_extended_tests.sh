#!/bin/bash
# Remote Code Execution Test Cases (CWE-94)
# pipe-to-bash, eval of fetched content, plugin downloads, remote execution primitives

# vuln-code-snippet start rce_pip_install_url
fetch_and_install() {
    local registry_url="$1"
    local pkg
    pkg=$(curl -s "${registry_url}/latest")
    pip install "$pkg"  # vuln-code-snippet vuln-line rce_pip_install_url
}
# vuln-code-snippet end rce_pip_install_url

# vuln-code-snippet start rce_npm_install_url
install_latest_npm() {
    local registry="$1"
    local pkg
    pkg=$(wget -qO- "${registry}/latest-package")
    npm install "$pkg"  # vuln-code-snippet vuln-line rce_npm_install_url
}
# vuln-code-snippet end rce_npm_install_url

# vuln-code-snippet start rce_gem_install_url
install_gem_release() {
    local url="$1"
    gem install "$(curl -s "$url")"  # vuln-code-snippet vuln-line rce_gem_install_url
}
# vuln-code-snippet end rce_gem_install_url

# vuln-code-snippet start rce_bash_ssh_remote
deploy_from_host() {
    local host="$1"
    bash <(ssh user@"$host" cat /deploy/script.sh)  # vuln-code-snippet vuln-line rce_bash_ssh_remote
}
# vuln-code-snippet end rce_bash_ssh_remote

# vuln-code-snippet start rce_eval_gpg_decrypt
apply_encrypted_config() {
    local payload_file="$1"
    eval "$(gpg --decrypt "$payload_file")"  # vuln-code-snippet vuln-line rce_eval_gpg_decrypt
}
# vuln-code-snippet end rce_eval_gpg_decrypt

# vuln-code-snippet start rce_docker_volume_escape
run_in_container() {
    local user_cmd="$1"
    docker run --rm -v /:/host busybox sh -c "$user_cmd"  # vuln-code-snippet vuln-line rce_docker_volume_escape
}
# vuln-code-snippet end rce_docker_volume_escape

# vuln-code-snippet start rce_kubectl_exec_user_cmd
exec_in_cluster() {
    local user_cmd="$1"
    kubectl exec -n prod deploy/app -- bash -c "$user_cmd"  # vuln-code-snippet vuln-line rce_kubectl_exec_user_cmd
}
# vuln-code-snippet end rce_kubectl_exec_user_cmd

# vuln-code-snippet start rce_ansible_shell_module
run_on_hosts() {
    local hosts_file="$1"
    local user_cmd="$2"
    ansible -i "$hosts_file" all -m shell -a "$user_cmd"  # vuln-code-snippet vuln-line rce_ansible_shell_module
}
# vuln-code-snippet end rce_ansible_shell_module

# vuln-code-snippet start rce_ssh_process_sub
deploy_via_ssh() {
    local host="$1"
    local setup_url="$2"
    ssh user@"$host" "bash -s" < <(curl -s "$setup_url")  # vuln-code-snippet vuln-line rce_ssh_process_sub
}
# vuln-code-snippet end rce_ssh_process_sub

# vuln-code-snippet start rce_git_clone_exec
install_plugin() {
    local repo="$1"
    git clone "$repo" /tmp/plugin
    bash /tmp/plugin/install.sh  # vuln-code-snippet vuln-line rce_git_clone_exec
}
# vuln-code-snippet end rce_git_clone_exec

# vuln-code-snippet start rce_docker_pull_run
run_user_image() {
    local user_image="$1"
    docker pull "$user_image"
    docker run --rm "$user_image" /entrypoint.sh  # vuln-code-snippet vuln-line rce_docker_pull_run
}
# vuln-code-snippet end rce_docker_pull_run

# vuln-code-snippet start rce_save_then_run_unverified
run_setup() {
    local setup_url="$1"
    local setup_script
    setup_script=$(mktemp)
    curl -fsSL "$setup_url" > "$setup_script"
    chmod +x "$setup_script"
    "$setup_script"  # vuln-code-snippet vuln-line rce_save_then_run_unverified
}
# vuln-code-snippet end rce_save_then_run_unverified

# vuln-code-snippet start rce_sudo_python_c_url
apply_system_config() {
    local config_url="$1"
    sudo python3 -c "$(curl -s "$config_url")"  # vuln-code-snippet vuln-line rce_sudo_python_c_url
}
# vuln-code-snippet end rce_sudo_python_c_url

# vuln-code-snippet start rce_plugin_url_pipe
load_plugin() {
    local plugin_name="$1"
    local plugin_url="https://plugins.example.com/${plugin_name}.sh"
    curl -s "$plugin_url" | bash  # vuln-code-snippet vuln-line rce_plugin_url_pipe
}
# vuln-code-snippet end rce_plugin_url_pipe

# vuln-code-snippet start rce_terraform_var_script
apply_infrastructure() {
    local user_url="$1"
    terraform apply -auto-approve -var "init_script=$user_url"  # vuln-code-snippet vuln-line rce_terraform_var_script
}
# vuln-code-snippet end rce_terraform_var_script

# --- True Negatives ---

# vuln-code-snippet start rce_download_verify_exec
install_verified() {
    local url="$1"
    wget -q "$url" -O /tmp/script.sh
    wget -q "${url}.sig" -O /tmp/script.sh.sig
    gpg --verify /tmp/script.sh.sig /tmp/script.sh
    bash /tmp/script.sh  # vuln-code-snippet safe-line rce_download_verify_exec
}
# vuln-code-snippet end rce_download_verify_exec

# vuln-code-snippet start rce_pip_require_hashes
install_dependencies() {
    pip install --require-hashes -r requirements.txt  # vuln-code-snippet safe-line rce_pip_require_hashes
}
# vuln-code-snippet end rce_pip_require_hashes

# vuln-code-snippet start rce_npm_ci_lockfile
install_node_deps() {
    npm ci  # vuln-code-snippet safe-line rce_npm_ci_lockfile
}
# vuln-code-snippet end rce_npm_ci_lockfile

# vuln-code-snippet start rce_docker_pinned_digest
pull_base_image() {
    docker pull "company.registry.io/app@sha256:abc123def456ee789abcdef0123456789abcdef0123456789abcdef01234567"  # vuln-code-snippet safe-line rce_docker_pinned_digest
}
# vuln-code-snippet end rce_docker_pinned_digest

# vuln-code-snippet start rce_go_install_pinned
install_tool() {
    go install github.com/example/staticcheck@v0.3.3  # vuln-code-snippet safe-line rce_go_install_pinned
}
# vuln-code-snippet end rce_go_install_pinned

# vuln-code-snippet start rce_cosign_verify
deploy_verified_image() {
    local image="company.registry.io/app:v1.2.3"
    cosign verify --certificate-identity=ci@example.com "$image"
    docker pull "$image"  # vuln-code-snippet safe-line rce_cosign_verify
}
# vuln-code-snippet end rce_cosign_verify

# vuln-code-snippet start rce_helm_pinned_chart
deploy_helm_chart() {
    helm install myapp oci://registry/charts/myapp --version 1.2.3  # vuln-code-snippet safe-line rce_helm_pinned_chart
}
# vuln-code-snippet end rce_helm_pinned_chart

# vuln-code-snippet start rce_local_script_hardcoded
run_deploy() {
    bash /var/app/scripts/deploy.sh  # vuln-code-snippet safe-line rce_local_script_hardcoded
}
# vuln-code-snippet end rce_local_script_hardcoded

# vuln-code-snippet start rce_bash_n_syntax_check
validate_script() {
    local script="$1"
    bash -n "$script"  # vuln-code-snippet safe-line rce_bash_n_syntax_check
}
# vuln-code-snippet end rce_bash_n_syntax_check

# vuln-code-snippet start rce_venv_pip_requirements
setup_python_env() {
    python3 -m venv venv
    venv/bin/pip install -r requirements.txt  # vuln-code-snippet safe-line rce_venv_pip_requirements
}
# vuln-code-snippet end rce_venv_pip_requirements

# vuln-code-snippet start rce_sha256_verify_before_exec
install_with_checksum() {
    local url="$1"
    curl -fsSL "$url" -o /tmp/install.sh
    curl -fsSL "${url}.sha256" -o /tmp/install.sh.sha256
    sha256sum -c /tmp/install.sh.sha256
    bash /tmp/install.sh  # vuln-code-snippet safe-line rce_sha256_verify_before_exec
}
# vuln-code-snippet end rce_sha256_verify_before_exec

# vuln-code-snippet start rce_ansible_playbook_check
validate_playbook() {
    ansible-playbook --check -i hosts site.yml  # vuln-code-snippet safe-line rce_ansible_playbook_check
}
# vuln-code-snippet end rce_ansible_playbook_check

# vuln-code-snippet start rce_kubectl_apply_hardcoded
apply_manifests() {
    kubectl apply -f /var/app/manifests/deployment.yaml  # vuln-code-snippet safe-line rce_kubectl_apply_hardcoded
}
# vuln-code-snippet end rce_kubectl_apply_hardcoded

# vuln-code-snippet start rce_cat_trusted_pipe
run_setup_script() {
    cat /var/app/trusted_setup.sh | bash  # vuln-code-snippet safe-line rce_cat_trusted_pipe
}
# vuln-code-snippet end rce_cat_trusted_pipe

# vuln-code-snippet start rce_gem_local_gemfile
install_ruby_deps() {
    bundle install --deployment  # vuln-code-snippet safe-line rce_gem_local_gemfile
}
# vuln-code-snippet end rce_gem_local_gemfile
