"""Webhook delivery service -- IDENTICAL between vuln/ and safe/ variants.

Flask application for a notification system that fires HTTP webhooks when
events occur. Users register webhook URLs; the runner delivers payloads to
those URLs via curl executed as a subprocess. If the stored URL contains
shell metacharacters, the curl invocation is vulnerable to command injection.

Chain: stored URL with shell metacharacters -> webhook_runner shell=True -> RCE
Individual findings: none in this file
Chain finding: webhook URL injection enables OS command execution (critical, CWE-78)
"""
from flask import Flask
from webhook_config import config_bp
from webhook_runner import runner_bp

app = Flask(__name__)
app.register_blueprint(config_bp)
app.register_blueprint(runner_bp)


@app.route("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)
