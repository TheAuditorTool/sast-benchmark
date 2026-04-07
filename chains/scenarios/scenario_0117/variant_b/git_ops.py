import subprocess
import os
from flask import Blueprint, jsonify, request

git_bp = Blueprint("git", __name__)

REPO_DIR = os.environ.get("REPO_DIR", "/var/repos/project")

def _run_git(args: list[str]) -> dict:
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
    return jsonify(_run_git(["status", "--short"])), 200

@git_bp.route("/repo/commit", methods=["POST"])
def repo_commit():
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
    result = _run_git(["gc", "--quiet"])
    return jsonify({"status": "gc complete", "details": result}), 200
