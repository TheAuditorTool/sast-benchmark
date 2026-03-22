#!/bin/bash
# Database Migration Script for Pipeline Manager
# Handles schema migrations with deep taint flows

set -e

# ============================================================================
# Configuration
# ============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "${SCRIPT_DIR}")"

# shellcheck source=../lib/utils.sh
. "${PROJECT_ROOT}/lib/utils.sh"
# shellcheck source=../lib/database.sh
. "${PROJECT_ROOT}/lib/database.sh"

readonly MIGRATIONS_DIR="${PROJECT_ROOT}/data/migrations"
readonly DB_FILE="${PROJECT_ROOT}/data/pipeline.db"

# ============================================================================
# Migration Functions
# ============================================================================
run_migration() {
    local direction="${1:-up}"
    local target="${2:-latest}"

    log_info "Running migration: ${direction} to ${target}"

    # Ensure migrations directory exists
    mkdir -p "${MIGRATIONS_DIR}"

    case "${direction}" in
        up)
            migrate_up "${target}"
            ;;
        down)
            migrate_down "${target}"
            ;;
        status)
            show_migration_status
            ;;
        create)
            create_migration "${target}"
            ;;
        *)
            log_error "Unknown migration direction: ${direction}"
            exit 1
            ;;
    esac
}

# vuln-code-snippet start sourceMigrationFile
migrate_up() {
    local target="$1"

    log_info "Migrating up to ${target}"

    # Get list of pending migrations
    local pending_migrations
    pending_migrations=$(get_pending_migrations)

    if [[ -z "${pending_migrations}" ]]; then
        log_info "No pending migrations"
        return 0
    fi

    # DEEP TAINT FLOW: User input (target) flows through multiple functions
    for migration in ${pending_migrations}; do
        local migration_id
        migration_id=$(basename "${migration}" .sh)

        if [[ "${target}" != "latest" ]] && [[ "${migration_id}" > "${target}" ]]; then
            log_debug "Skipping ${migration_id} (target: ${target})"
            continue
        fi

        log_info "Applying migration: ${migration_id}"

        # Execute migration
        source "${migration}"  # vuln-code-snippet vuln-line sourceMigrationFile

        # Call the up function
        if declare -f migration_up > /dev/null; then
            migration_up
        else
            log_error "Migration ${migration_id} missing migration_up function"
            exit 1
        fi

        # Record migration
        record_migration "${migration_id}" "$(hash_file_sha256 "${migration}")"

        log_info "Migration ${migration_id} applied"

        # Clean up function definitions
        unset -f migration_up migration_down 2>/dev/null || true
    done

    log_info "All migrations completed"
}
# vuln-code-snippet end sourceMigrationFile

# vuln-code-snippet start sqlInjectionDeleteMigration
migrate_down() {
    local target="$1"

    log_info "Migrating down to ${target}"

    # Get applied migrations in reverse order
    local applied_migrations
    applied_migrations=$(get_applied_migrations | tac)

    for migration_id in ${applied_migrations}; do
        if [[ "${target}" != "0" ]] && [[ ! "${migration_id}" > "${target}" ]]; then
            log_debug "Stopping at ${migration_id} (target: ${target})"
            break
        fi

        local migration_file="${MIGRATIONS_DIR}/${migration_id}.sh"

        if [[ ! -f "${migration_file}" ]]; then
            log_warn "Migration file not found: ${migration_file}"
            continue
        fi

        log_info "Rolling back migration: ${migration_id}"

        # Execute migration rollback
        source "${migration_file}"

        if declare -f migration_down > /dev/null; then
            migration_down
        else
            log_warn "Migration ${migration_id} missing migration_down function"
        fi

        # Remove migration record
        db_execute "DELETE FROM migrations WHERE migration_id = '${migration_id}'"  # vuln-code-snippet vuln-line sqlInjectionDeleteMigration

        log_info "Migration ${migration_id} rolled back"

        unset -f migration_up migration_down 2>/dev/null || true
    done
}
# vuln-code-snippet end sqlInjectionDeleteMigration

get_pending_migrations() {
    local all_migrations
    all_migrations=$(find "${MIGRATIONS_DIR}" -name "*.sh" -type f | sort)

    for migration in ${all_migrations}; do
        local migration_id
        migration_id=$(basename "${migration}" .sh)

        if ! is_migration_applied "${migration_id}"; then
            echo "${migration}"
        fi
    done
}

show_migration_status() {
    log_info "Migration Status:"
    echo "=================="

    local all_migrations
    all_migrations=$(find "${MIGRATIONS_DIR}" -name "*.sh" -type f 2>/dev/null | sort)

    if [[ -z "${all_migrations}" ]]; then
        echo "No migrations found"
        return
    fi

    for migration in ${all_migrations}; do
        local migration_id
        migration_id=$(basename "${migration}" .sh)

        if is_migration_applied "${migration_id}"; then
            echo "[x] ${migration_id}"
        else
            echo "[ ] ${migration_id}"
        fi
    done
}

create_migration() {
    local name="$1"

    if [[ -z "${name}" ]]; then
        log_error "Usage: migrate.sh create <migration_name>"
        exit 1
    fi

    local timestamp
    timestamp=$(date +%Y%m%d%H%M%S)

    local migration_id="${timestamp}_${name}"
    local migration_file="${MIGRATIONS_DIR}/${migration_id}.sh"

    cat > "${migration_file}" << 'MIGRATION_TEMPLATE'
#!/bin/bash
# Migration: MIGRATION_ID
# Created: TIMESTAMP

migration_up() {
    # TODO: Implement forward migration
    sqlite3 "${DB_FILE}" << 'SQL'
-- Add your migration SQL here
SQL
}

migration_down() {
    # TODO: Implement rollback
    sqlite3 "${DB_FILE}" << 'SQL'
-- Add your rollback SQL here
SQL
}
MIGRATION_TEMPLATE

    # Replace placeholders
    sed -i "s/MIGRATION_ID/${migration_id}/g" "${migration_file}"
    sed -i "s/TIMESTAMP/$(date)/g" "${migration_file}"

    chmod +x "${migration_file}"

    log_info "Created migration: ${migration_file}"
}

# ============================================================================
# Advanced Migration Features
# ============================================================================

# vuln-code-snippet start readWithoutRMigration
# DEEP TAINT FLOW: Data flows from file -> variable -> database -> command
process_migration_data() {
    local data_file="$1"

    if [[ ! -f "${data_file}" ]]; then
        log_error "Data file not found: ${data_file}"
        return 1
    fi

    # TAINT SOURCE: Read from file
    local data
    data=$(cat "${data_file}")

    # TAINT PROPAGATION: Process data line by line
    echo "${data}" | while read line; do  # vuln-code-snippet vuln-line readWithoutRMigration
        # Parse line
        local key value
        key=$(echo "${line}" | cut -d: -f1)
        value=$(echo "${line}" | cut -d: -f2-)
# vuln-code-snippet end readWithoutRMigration

# vuln-code-snippet start sqlInjectionMigrationData
        # TAINT SINK: Insert into database (SQL injection risk)
        db_execute "INSERT INTO migration_data (key, value) VALUES ('${key}', '${value}')"  # vuln-code-snippet vuln-line sqlInjectionMigrationData
    done
}
# vuln-code-snippet end sqlInjectionMigrationData

# vuln-code-snippet start executeCustomSqlFromStdin
# DEEP TAINT FLOW: User input -> SQL -> command execution
execute_custom_sql() {
    local sql_file="$1"

    if [[ -z "${sql_file}" ]]; then
        # Read from stdin
        local sql
        read sql
    else
        local sql
        sql=$(cat "${sql_file}")
    fi

    # TAINT SINK: Execute user-provided SQL
    log_info "Executing custom SQL"
    sqlite3 "${DB_FILE}" "${sql}"  # vuln-code-snippet vuln-line executeCustomSqlFromStdin
}
# vuln-code-snippet end executeCustomSqlFromStdin

# ============================================================================
# Migration Hooks
# ============================================================================
# vuln-code-snippet start sourcePreMigrationHook
run_pre_migration_hook() {
    local migration_id="$1"

    local hook_file="${MIGRATIONS_DIR}/hooks/pre_${migration_id}.sh"

    if [[ -f "${hook_file}" ]]; then
        log_info "Running pre-migration hook: ${migration_id}"
        . "${hook_file}"  # vuln-code-snippet vuln-line sourcePreMigrationHook
    fi
}
# vuln-code-snippet end sourcePreMigrationHook

run_post_migration_hook() {
    local migration_id="$1"

    local hook_file="${MIGRATIONS_DIR}/hooks/post_${migration_id}.sh"

    if [[ -f "${hook_file}" ]]; then
        log_info "Running post-migration hook: ${migration_id}"
        source "${hook_file}"
    fi
}

# ============================================================================
# Seed Data
# ============================================================================
seed_database() {
    local seed_file="${1:-${PROJECT_ROOT}/data/seeds.sql}"

    if [[ ! -f "${seed_file}" ]]; then
        log_error "Seed file not found: ${seed_file}"
        return 1
    fi

    log_info "Seeding database from ${seed_file}"

    sqlite3 "${DB_FILE}" < "${seed_file}"

    log_info "Database seeded"
}

# vuln-code-snippet start runSeedScript
run_seed_script() {
    local seed_name="$1"

    local seed_script="${PROJECT_ROOT}/data/seeds/${seed_name}.sh"

    if [[ -f "${seed_script}" ]]; then
        "${seed_script}"  # vuln-code-snippet vuln-line runSeedScript
    else
        log_error "Seed script not found: ${seed_name}"
        return 1
    fi
}
# vuln-code-snippet end runSeedScript

# ============================================================================
# Entry Point
# ============================================================================
main() {
    local direction="${1:-up}"
    local target="${2:-latest}"

    run_migration "${direction}" "${target}"
}

main "$@"
