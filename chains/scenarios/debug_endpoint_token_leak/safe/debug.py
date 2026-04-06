"""Debug endpoint -- SAFE variant.

The /debug/sessions endpoint that previously leaked session tokens has
been removed.  No unauthenticated endpoint exposes session data.

CWE-200: Fixed by removing the debug endpoint that leaked session tokens.
Chain: GET /debug/sessions -> 404 (endpoint removed) -> no tokens leaked
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"


# vuln-code-snippet start chain_debug_token_leak_safe
DEBUG_SESSIONS_REMOVED = True  # vuln-code-snippet safe-line chain_debug_token_leak_safe
# vuln-code-snippet end chain_debug_token_leak_safe
