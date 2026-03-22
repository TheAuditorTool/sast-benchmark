#!/bin/bash
# Secure Pipeline — Deployment Operations (Hardened)
# All functions demonstrate SAFE deployment patterns.
# Defense strategies: quoting, integer validation, allowlists, strict SSH.

DEPLOY_DIR="${DEPLOY_DIR:-/var/securepipeline/deployments}"

# vuln-code-snippet start sp_deploy_ssh_strict
deploy_to_target_safe() {
    # Safe: SSH uses StrictHostKeyChecking=yes (the default, but explicitly set).
    # This prevents MITM attacks by rejecting unknown host keys.
    local host="$1"
    local version="$2"

    ssh -o StrictHostKeyChecking=yes -o BatchMode=yes "$host" \
        "systemctl restart app"  # vuln-code-snippet safe-line sp_deploy_ssh_strict
}
# vuln-code-snippet end sp_deploy_ssh_strict

# vuln-code-snippet start sp_deploy_ssh_quoted_args
run_remote_command_safe() {
    # Safe: the remote command is a fixed string. The version argument is
    # integer-validated before being interpolated into the service name.
    # No user-controlled data reaches a shell interpreter.
    local host="$1"
    local version="$2"

    if [[ ! "$version" =~ ^[0-9]+$ ]]; then
        echo "Invalid version: must be integer" >&2
        return 1
    fi

    ssh "$host" "systemctl restart app-${version}"  # vuln-code-snippet safe-line sp_deploy_ssh_quoted_args
}
# vuln-code-snippet end sp_deploy_ssh_quoted_args

# vuln-code-snippet start sp_deploy_docker_quoted
docker_run_safe() {
    # Safe: all variables are properly double-quoted in docker run.
    # Quoting prevents word splitting and glob expansion on container names,
    # env files, and image names that contain spaces or special characters.
    local container_name="$1"
    local env_file="$2"
    local image="$3"

    docker run -d \
        --name "$container_name" \
        --env-file "$env_file" \
        --restart unless-stopped \
        "$image"  # vuln-code-snippet safe-line sp_deploy_docker_quoted
}
# vuln-code-snippet end sp_deploy_docker_quoted

# vuln-code-snippet start sp_deploy_chmod_755
prepare_deploy_directory_safe() {
    # Safe: chmod 755 gives owner rwx, group/world rx.
    # This is appropriate for deployment directories that need to be
    # readable by the web server but only writable by the deployer.
    local deploy_path="$1"

    mkdir -p "$deploy_path"
    chmod 755 "$deploy_path"  # vuln-code-snippet safe-line sp_deploy_chmod_755
}
# vuln-code-snippet end sp_deploy_chmod_755

# vuln-code-snippet start sp_deploy_quoted_cp
copy_artifacts_safe() {
    # Safe: both source and destination paths are double-quoted.
    # This prevents word splitting on paths containing spaces.
    local src="$1"
    local dst="$2"

    cp -r "$src" "$dst"  # vuln-code-snippet safe-line sp_deploy_quoted_cp
}
# vuln-code-snippet end sp_deploy_quoted_cp

# vuln-code-snippet start sp_deploy_version_integer
validate_and_deploy() {
    # Safe: version string is validated against semantic versioning regex
    # before any use. Only digits and dots in a structured format pass.
    local version="$1"
    local target="$2"

    if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Invalid version format (expected X.Y.Z): $version" >&2
        return 1
    fi

    echo "Deploying version $version to $target"  # vuln-code-snippet safe-line sp_deploy_version_integer
    cp -r "${DEPLOY_DIR}/releases/${version}" "${DEPLOY_DIR}/current"
}
# vuln-code-snippet end sp_deploy_version_integer

# vuln-code-snippet start sp_deploy_hook_allowlist
run_deploy_hook_safe() {
    # Safe: hook name validated against a fixed allowlist. Only pre-deploy,
    # post-deploy, and rollback hooks can be executed. No arbitrary commands.
    local hook_name="$1"
    local hook_dir="${DEPLOY_DIR}/hooks"

    case "$hook_name" in
        pre-deploy|post-deploy|rollback)
            if [[ -x "${hook_dir}/${hook_name}.sh" ]]; then
                "${hook_dir}/${hook_name}.sh"  # vuln-code-snippet safe-line sp_deploy_hook_allowlist
            fi
            ;;
        *)
            echo "Unknown hook: $hook_name" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end sp_deploy_hook_allowlist

# vuln-code-snippet start sp_deploy_kubectl_identifier
kubectl_rollout_safe() {
    # Safe: namespace and deployment name are validated against a strict
    # identifier regex. Only lowercase alphanumeric with hyphens allowed.
    # This prevents argument injection via kubectl.
    local namespace="$1"
    local deployment="$2"

    if [[ ! "$namespace" =~ ^[a-z][a-z0-9-]*$ ]]; then
        echo "Invalid namespace: $namespace" >&2
        return 1
    fi
    if [[ ! "$deployment" =~ ^[a-z][a-z0-9-]*$ ]]; then
        echo "Invalid deployment: $deployment" >&2
        return 1
    fi

    kubectl rollout restart "deployment/${deployment}" -n "$namespace"  # vuln-code-snippet safe-line sp_deploy_kubectl_identifier
}
# vuln-code-snippet end sp_deploy_kubectl_identifier
