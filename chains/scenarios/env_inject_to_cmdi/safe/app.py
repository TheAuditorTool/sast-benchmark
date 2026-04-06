"""Build task execution service -- IDENTICAL between vuln/ and safe/ variants.

Flask application for a CI/CD helper that runs build commands in a subprocess
with a configurable environment. The environment dictionary passed to the child
process is assembled from user input. If the child process (or any tool it
invokes) reads environment variables to influence behavior, an attacker can
inject hostile values that lead to command execution.

Chain: user-controlled env vars passed to subprocess -> child reads tainted env -> RCE
Individual findings: none in this file
Chain finding: environment variable injection enables OS command execution (critical, CWE-78)
"""
from flask import Flask
from process_manager import pm_bp
from env_config import env_bp

app = Flask(__name__)
app.register_blueprint(pm_bp)
app.register_blueprint(env_bp)


@app.route("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)
