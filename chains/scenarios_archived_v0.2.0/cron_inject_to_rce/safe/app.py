"""Scheduled task service -- IDENTICAL between vuln/ and safe/ variants.

Flask application for a job scheduling service where authenticated users
can schedule recurring tasks (e.g., report generation, data exports).
Task schedules are written to the system crontab; the task command is
then executed by cron at the specified interval.

Chain: unsanitized user input in crontab entry -> cron executes injected command -> RCE
Individual findings: none in this file
Chain finding: cron injection enables OS command execution (critical, CWE-78)
"""
from flask import Flask
from scheduler import scheduler_bp
from task_runner import runner_bp

app = Flask(__name__)
app.register_blueprint(scheduler_bp)
app.register_blueprint(runner_bp)


@app.route("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)
