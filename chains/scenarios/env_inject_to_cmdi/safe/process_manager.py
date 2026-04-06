"""Process manager -- SAFE variant.

Runs build commands in a subprocess with an explicitly constructed environment
containing only allowlisted variable names. Instead of passing the full
user-controlled ENV_STORE, this module copies only variables whose names appear
in SAFE_ENV_KEYS into the subprocess environment. Dangerous variables like
LD_PRELOAD, PYTHONSTARTUP, BASH_ENV, GIT_SSH_COMMAND, and PATH cannot be
set by an attacker because they are not in the allowlist.

Chain: POST /env/set {"LD_PRELOAD": "/tmp/evil.so"} -> ENV_STORE updated ->
       POST /build/run -> build_safe_env() skips LD_PRELOAD (not allowlisted) ->
       subprocess launched without attacker variable -> no library injection
Individual findings: none -- env filtered to allowlist before subprocess launch
Chain finding: none -- dangerous env var injection prevented (CWE-78 mitigated)
"""
import subprocess
import os
from flask import Blueprint, request, jsonify
from env_config import ENV_STORE

pm_bp = Blueprint("pm", __name__)

ALLOWED_COMMANDS = frozenset(["make", "cmake", "ninja", "gradle", "mvn", "cargo"])

SAFE_ENV_KEYS = frozenset([
    "BUILD_TARGET",
    "BUILD_PROFILE",
    "JAVA_HOME",
    "CARGO_TARGET_DIR",
    "CMAKE_BUILD_TYPE",
    "GRADLE_USER_HOME",
    "MVN_OPTS",
    "MAKE_FLAGS",
])


def build_safe_env() -> dict[str, str]:
    """Construct a subprocess environment from allowlisted ENV_STORE keys only."""
    safe = {}
    for key in SAFE_ENV_KEYS:
        if key in ENV_STORE:
            safe[key] = ENV_STORE[key]
    return safe


# vuln-code-snippet start chain_env_inject_cmdi_safe
@pm_bp.route("/build/run", methods=["POST"])
def run_build():
    """Execute a build command with a filtered environment.

    Expects JSON with 'command' (base command name) and optional 'args' list.
    Only allowlisted environment variable names from ENV_STORE are passed
    to the subprocess; all other user-supplied variables are dropped.
    """
    body = request.get_json(silent=True) or {}
    command = body.get("command", "")
    args = body.get("args", [])

    if command not in ALLOWED_COMMANDS:
        return jsonify({"error": f"Command not allowed: {command}"}), 400
    if not isinstance(args, list):
        return jsonify({"error": "args must be a list"}), 400

    cmd_list = [command] + [str(a) for a in args]
    result = subprocess.run(
        cmd_list,
        capture_output=True,
        text=True,
        timeout=300,
        env=build_safe_env(),  # vuln-code-snippet safe-line chain_env_inject_cmdi_safe
    )
    return jsonify({
        "command": cmd_list,
        "returncode": result.returncode,
        "stdout": result.stdout.strip(),
        "stderr": result.stderr.strip(),
    }), 200
# vuln-code-snippet end chain_env_inject_cmdi_safe
