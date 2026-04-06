"""Git repository management service -- IDENTICAL between vuln/ and safe/ variants.

Flask application for a self-hosted git service that allows project maintainers
to upload configuration files and trigger repository operations.

Chain: path traversal in file write -> .git/hooks/ overwritten -> git op triggers hook -> RCE
Individual findings: none in this file
Chain finding: unauthenticated RCE via git hook injection (critical, CWE-78)
"""
from flask import Flask
from file_manager import files_bp
from git_ops import git_bp

app = Flask(__name__)
app.register_blueprint(files_bp)
app.register_blueprint(git_bp)


@app.route("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)
