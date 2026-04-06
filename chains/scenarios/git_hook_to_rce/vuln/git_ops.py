"""Git operations module -- IDENTICAL between vuln/ and safe/ variants.

Provides endpoints for triggering repository operations (status, commit, gc).
Git automatically executes scripts in .git/hooks/ at key lifecycle points --
pre-commit, post-commit, pre-receive, update, etc. If an attacker can write
to .git/hooks/ (via the path-traversal bug in file_manager.py), any hook
script placed there will be executed as the web server user when the
corresponding git operation runs.

Chain: file_manager.py writes attacker payload to .git/hooks/pre-commit ->
       POST /repo/commit triggers `git commit` -> git executes hook -> RCE
Individual findings: none (git subprocess is a legitimate operation)
Chain finding: hook execution after traversal write enables RCE (critical, CWE-78)
"""
import subprocess
import os
from flask import Blueprint, jsonify, request

git_bp = Blueprint("git", __name__)

REPO_DIR = os.environ.get("REPO_DIR", "/var/repos/project")


def _run_git(args: list[str]) -> dict:
    """Run a git command inside REPO_DIR and return stdout/stderr."""
    result = subprocess.run(
        ["git"] + args,
        cwd=REPO_DIR,
        capture_output=True,
        text=True,
        timeout=30,
    )
    return {
        "returncode": result.returncode,
        "stdout": result.stdout.strip(),
        "stderr": result.stderr.strip(),
    }


@git_bp.route("/repo/status", methods=["GET"])
def repo_status():
    """Return the working-tree status of the repository."""
    return jsonify(_run_git(["status", "--short"])), 200


@git_bp.route("/repo/commit", methods=["POST"])
def repo_commit():
    """Stage all changes and create a commit with the provided message."""
    body = request.get_json(silent=True) or {}
    message = body.get("message", "automated commit")
    if not isinstance(message, str) or not message.strip():
        return jsonify({"error": "commit message required"}), 400
    _run_git(["add", "-A"])
    result = _run_git(["commit", "-m", message])
    if result["returncode"] not in (0, 1):
        return jsonify({"error": "commit failed", "details": result}), 500
    return jsonify({"status": "committed", "details": result}), 200


@git_bp.route("/repo/gc", methods=["POST"])
def repo_gc():
    """Run git garbage collection to prune loose objects."""
    result = _run_git(["gc", "--quiet"])
    return jsonify({"status": "gc complete", "details": result}), 200
