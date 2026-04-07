"""OAuth redirect_uri validation -- SAFE variant.

validate_redirect_uri() checks the supplied URI against a per-client
allowlist.  An unregistered URI is rejected with a 400 before any code is issued.

Chain: /oauth/authorize?redirect_uri=https://evil.com -> 400 (not in allowlist) -> blocked
"""

REGISTERED_URIS = {
    "client_app_1": {"https://app.example.com/callback"},
    "client_app_2": {"https://partner.example.com/oauth/callback"},
}


def validate_redirect_uri(redirect_uri, client_id):
    """Return True only if redirect_uri is registered for the client."""
    allowed = REGISTERED_URIS.get(client_id, set())
    return redirect_uri in allowed
