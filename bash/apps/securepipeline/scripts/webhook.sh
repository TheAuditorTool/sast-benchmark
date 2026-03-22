#!/bin/bash
# Secure Pipeline — Webhook Handler (Hardened)
# All functions demonstrate SAFE webhook processing patterns.
# Defense strategies: input validation, SQL parameter safety, allowlists,
# hardcoded URLs, HMAC verification.

DB_FILE="${DB_FILE:-/var/securepipeline/deployments.db}"
CI_ENDPOINT="https://ci.corp.internal/api/v1/builds"

# vuln-code-snippet start sp_webhook_sql_validated
handle_push_event_safe() {
    # Safe: branch name and commit SHA are validated against strict regexes
    # before use in SQL. Branch allows alphanumeric/dots/slashes/hyphens.
    # Commit SHA is validated as a 40-character hex string.
    local branch="$1"
    local commit_sha="$2"

    if [[ ! "$branch" =~ ^[a-zA-Z0-9._/-]+$ ]]; then
        echo "Invalid branch name" >&2
        return 1
    fi
    if [[ ! "$commit_sha" =~ ^[0-9a-f]{40}$ ]]; then
        echo "Invalid commit SHA" >&2
        return 1
    fi

    sqlite3 "$DB_FILE" "INSERT INTO webhook_events (branch, commit_sha, event_type) VALUES ('${branch}', '${commit_sha}', 'push')"  # vuln-code-snippet safe-line sp_webhook_sql_validated
}
# vuln-code-snippet end sp_webhook_sql_validated

# vuln-code-snippet start sp_webhook_tag_validated
handle_release_event_safe() {
    # Safe: tag name is validated against semantic version format.
    # Only vN.N.N patterns pass, preventing SQL injection via tag names.
    local tag_name="$1"

    if [[ ! "$tag_name" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Invalid tag format (expected vX.Y.Z): $tag_name" >&2
        return 1
    fi

    sqlite3 "$DB_FILE" "INSERT INTO releases (tag, created_at) VALUES ('${tag_name}', datetime('now'))"  # vuln-code-snippet safe-line sp_webhook_tag_validated
}
# vuln-code-snippet end sp_webhook_tag_validated

# vuln-code-snippet start sp_webhook_cmdi_allowlisted
execute_deploy_command_safe() {
    # Safe: service name is validated against a fixed allowlist of known
    # services. The deploy script path is a hardcoded constant.
    local service="$1"
    local version="$2"
    local deploy_script="/opt/securepipeline/deploy.sh"

    case "$service" in
        webapp|api|worker|scheduler)
            "$deploy_script" "$service" "$version"  # vuln-code-snippet safe-line sp_webhook_cmdi_allowlisted
            ;;
        *)
            echo "Unknown service: $service" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end sp_webhook_cmdi_allowlisted

# vuln-code-snippet start sp_webhook_deploy_validated
handle_deploy_event_safe() {
    # Safe: both environment and version are validated before use.
    # Environment is allowlisted. Version is regex-validated.
    local environment="$1"
    local version="$2"

    case "$environment" in
        staging|production) ;;
        *)
            echo "Invalid environment: $environment" >&2
            return 1
            ;;
    esac

    if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Invalid version: $version" >&2
        return 1
    fi

    /opt/securepipeline/pipeline.sh deploy "$environment" "$version"  # vuln-code-snippet safe-line sp_webhook_deploy_validated
}
# vuln-code-snippet end sp_webhook_deploy_validated

# vuln-code-snippet start sp_webhook_kubectl_validated
cleanup_pr_environment_safe() {
    # Safe: PR number is validated as a pure integer before constructing
    # the Kubernetes namespace name. Integer values cannot inject kubectl args.
    local pr_number="$1"

    if [[ ! "$pr_number" =~ ^[0-9]+$ ]]; then
        echo "Invalid PR number" >&2
        return 1
    fi

    kubectl delete namespace "pr-${pr_number}"  # vuln-code-snippet safe-line sp_webhook_kubectl_validated
}
# vuln-code-snippet end sp_webhook_kubectl_validated

# vuln-code-snippet start sp_webhook_ssrf_hardcoded
trigger_ci_build_safe() {
    # Safe: CI endpoint URL is a hardcoded constant. User-provided branch
    # name is placed only in the JSON body (quoted), not in the URL.
    local branch="$1"
    local pr_number="$2"

    curl -sf -X POST \
        -H "Content-Type: application/json" \
        -d "{\"branch\":\"${branch}\",\"pr\":\"${pr_number}\"}" \
        "$CI_ENDPOINT"  # vuln-code-snippet safe-line sp_webhook_ssrf_hardcoded
}
# vuln-code-snippet end sp_webhook_ssrf_hardcoded

# vuln-code-snippet start sp_webhook_source_validated
load_webhook_handler_safe() {
    # Safe: handler type is validated against a fixed list before
    # constructing the source path. Only known handler types load.
    local handler_type="$1"
    local handler_dir="/opt/securepipeline/handlers"

    case "$handler_type" in
        github|gitlab|bitbucket|slack)
            source "${handler_dir}/${handler_type}.sh"  # vuln-code-snippet safe-line sp_webhook_source_validated
            ;;
        *)
            echo "Unknown handler type: $handler_type" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end sp_webhook_source_validated

# vuln-code-snippet start sp_webhook_hmac_sha256
verify_webhook_signature_safe() {
    # Safe: uses HMAC-SHA256 for webhook signature verification.
    # SHA-256 is cryptographically strong, unlike SHA-1 which has known
    # collision attacks. This follows GitHub's X-Hub-Signature-256 spec.
    local payload="$1"
    local provided_signature="$2"
    local secret="${WEBHOOK_SECRET:?WEBHOOK_SECRET must be set}"

    local expected
    expected=$(echo -n "$payload" | openssl dgst -sha256 -hmac "$secret" | awk '{print $2}')  # vuln-code-snippet safe-line sp_webhook_hmac_sha256

    if [[ "sha256=${expected}" != "$provided_signature" ]]; then
        echo "Invalid webhook signature" >&2
        return 1
    fi
}
# vuln-code-snippet end sp_webhook_hmac_sha256

# vuln-code-snippet start sp_webhook_no_eval
parse_query_string_safe() {
    # Safe: query string is parsed using read + IFS splitting, NOT eval.
    # Each key=value pair is stored in an associative array via direct
    # assignment. No shell interpretation of the values occurs.
    local query_string="$1"
    local -A params

    local pair key value
    while IFS='=' read -r -d '&' key value; do
        params["$key"]="$value"  # vuln-code-snippet safe-line sp_webhook_no_eval
    done <<< "${query_string}&"

    # Return specific param
    echo "${params[action]}"
}
# vuln-code-snippet end sp_webhook_no_eval
