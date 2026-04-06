"""Process manager -- VULNERABLE variant.

Runs build commands in a subprocess, passing the entire user-controlled
ENV_STORE as the subprocess environment. An attacker who sets LD_PRELOAD,
PYTHONSTARTUP, BASH_ENV, GIT_SSH_COMMAND, or PATH in the env store can
control the behaviour of the spawned process and any tools it invokes.

Because the full ENV_STORE (sourced entirely from user input) is passed as
the env argument, there is no filtering of dangerous variable names.

Chain: POST /env/set {"LD_PRELOAD": "/tmp/evil.so"} -> ENV_STORE updated ->
       POST /build/run -> subprocess launched with attacker-controlled env ->
       linker loads evil.so -> arbitrary code execution
Individual findings: user-controlled env dict passed to subprocess (high)
Chain finding: combined with unrestricted env store, enables library injection
               and OS command execution (critical, CWE-78)
"""
import subprocess
import os
from flask import Blueprint, request, jsonify
from env_config import ENV_STORE

pm_bp = Blueprint("pm", __name__)

ALLOWED_COMMANDS = frozenset(["make", "cmake", "ninja", "gradle", "mvn", "cargo"])


# vuln-code-snippet start chain_env_inject_cmdi_vuln
@pm_bp.route("/build/run", methods=["POST"])
def run_build():
    """Execute a build command with the configured environment.

    Expects JSON with 'command' (base command name) and optional 'args' list.
    The full ENV_STORE is passed as the subprocess environment without filtering.
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
        env=dict(ENV_STORE),  # vuln-code-snippet vuln-line chain_env_inject_cmdi_vuln
    )
    return jsonify({
        "command": cmd_list,
        "returncode": result.returncode,
        "stdout": result.stdout.strip(),
        "stderr": result.stderr.strip(),
    }), 200
# vuln-code-snippet end chain_env_inject_cmdi_vuln
