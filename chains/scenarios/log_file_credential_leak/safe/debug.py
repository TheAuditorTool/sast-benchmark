"""Request logger -- SAFE variant.

Logs only the request path, not the body, so credentials posted to
/login are never written to disk.  The /logs endpoint is also removed.

CWE-200: Fixed by redacting the request body from log entries.
Chain: POST /login -> body not logged -> GET /logs returns no credentials
"""
from config import app, LOG_FILE


def log_request(path, body):
    """Append only the request path to the log file (body redacted).

    SAFE: the body is never written; credentials cannot appear in the log.
    """
    with open(LOG_FILE, "a") as f:
        f.write(path + "\n")


# vuln-code-snippet start chain_log_cred_leak_safe
LOG_ENDPOINT_REMOVED = True  # vuln-code-snippet safe-line chain_log_cred_leak_safe
# vuln-code-snippet end chain_log_cred_leak_safe
