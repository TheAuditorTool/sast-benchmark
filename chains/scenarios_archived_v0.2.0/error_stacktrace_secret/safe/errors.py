"""Error handler -- SAFE variant.

On unhandled exceptions, returns a generic error message.  No traceback,
no configuration values, and no API keys are included in the response.

CWE-200: Fixed by replacing verbose error output with a generic message.
Chain: GET /crash -> exception -> generic error returned -> no secrets leaked
"""
from flask import Flask, jsonify

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"


# vuln-code-snippet start chain_error_stacktrace_safe
@app.errorhandler(Exception)
def handle_exception(exc):
    """Return a generic error message on any unhandled exception.

    SAFE: no traceback or configuration values are included.
    """
    return jsonify({"error": "An internal error occurred"}), 500  # vuln-code-snippet safe-line chain_error_stacktrace_safe
# vuln-code-snippet end chain_error_stacktrace_safe
