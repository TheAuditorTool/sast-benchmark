"""Token refresh logic -- VULNERABLE variant.

Validates the old token with a SELECT, then issues a new token and invalidates
the old one as separate operations.  Two concurrent refresh requests both validate
the same old token before either invalidates it -- each receives a distinct new
valid token, effectively multiplying a single valid token into many.

Chain: concurrent POST /refresh -> SELECT valid token -> [race window] -> issue new + invalidate old
Individual findings: non-atomic token rotation (medium)
Chain finding: token replay -- old token exchanged multiple times for multiple valid tokens (CWE-362, critical)
"""
from tokens import get_valid_token, create_token, invalidate_token


def refresh_token(old_token):
    """
    Exchange old_token for a new one.

    VULNERABLE: validation check and invalidation are separate operations.
    Concurrent refresh requests with the same old_token can both pass the check.
    """
    token_record = get_valid_token(old_token)

    # TOCTOU: another concurrent request can also pass this check with the same old token
    if not token_record:
        return {"error": "Invalid or expired token"}, 401

    user_id = token_record["user_id"]
    new_token = create_token(user_id)

# vuln-code-snippet start chain_token_refresh_race_vuln
    invalidate_token(old_token)  # vuln-code-snippet vuln-line chain_token_refresh_race_vuln
# vuln-code-snippet end chain_token_refresh_race_vuln

    return {"status": "ok", "token": new_token}, 200
