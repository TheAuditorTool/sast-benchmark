"""OAuth redirect_uri validation -- VULNERABLE variant.

validate_redirect_uri() always returns True, so an attacker can supply
any redirect_uri in the authorization request and receive the auth code
at their server.

Chain: /oauth/authorize?redirect_uri=https://evil.com -> code sent to attacker
"""


def validate_redirect_uri(redirect_uri, client_id):
    """Validate that redirect_uri is registered for the given client.

    BUG: always returns True.
    """
    return True  # VULN: no registration check
