"""Environment configuration module -- IDENTICAL between vuln/ and safe/ variants.

Manages a session-level environment variable store that is passed to build
subprocesses. Provides endpoints to set, get, and clear environment variables.
The values stored here flow directly into the subprocess environment in
process_manager.py.

Variables like LD_PRELOAD, PYTHONSTARTUP, GIT_SSH_COMMAND, PATH, and
BASH_ENV are particularly dangerous: a child process that reads these can
be hijacked to execute attacker-controlled code. The vulnerability is in
process_manager.py, which passes the entire user-controlled store to the
subprocess without filtering.

Chain: POST /env/set with LD_PRELOAD=/tmp/evil.so -> stored in ENV_STORE ->
       POST /build/run -> subprocess launched with LD_PRELOAD set ->
       dynamic linker loads attacker library -> RCE
Individual findings: none in isolation (env store is a legitimate feature)
Chain finding: combined with unfiltered env passthrough, enables library
               injection and command execution (critical, CWE-78)
"""
from flask import Blueprint, request, jsonify

env_bp = Blueprint("env", __name__)

ENV_STORE: dict[str, str] = {}


@env_bp.route("/env/set", methods=["POST"])
def set_env():
    """Set one or more environment variables in the build environment store."""
    body = request.get_json(silent=True)
    if not body or not isinstance(body, dict):
        return jsonify({"error": "JSON object required"}), 400
    for key, value in body.items():
        if not isinstance(key, str) or not isinstance(value, str):
            return jsonify({"error": "All keys and values must be strings"}), 400
        ENV_STORE[key] = value
    return jsonify({"status": "updated", "keys": list(ENV_STORE.keys())}), 200


@env_bp.route("/env/get", methods=["GET"])
def get_env():
    """Return the current build environment store."""
    return jsonify({"env": ENV_STORE}), 200


@env_bp.route("/env/clear", methods=["POST"])
def clear_env():
    """Clear all variables from the build environment store."""
    count = len(ENV_STORE)
    ENV_STORE.clear()
    return jsonify({"status": "cleared", "removed": count}), 200
