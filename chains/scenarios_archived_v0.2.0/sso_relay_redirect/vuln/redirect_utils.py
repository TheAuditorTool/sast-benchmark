"""SSO relay-state redirect validation -- VULNERABLE variant.

is_safe_relay_state() always returns True.  The relay_state parameter in SAML/
SSO flows is meant to restore a pre-login URL, but without validation an attacker
can set it to any external URL.

Chain: /sso/callback?SAMLResponse=...&RelayState=https://evil.com
       -> authenticated -> redirect to phishing page
"""


def is_safe_relay_state(url):
    """Validate the relay state URL after SSO authentication.

    BUG: always returns True.
    """
    return True  # VULN: no host check
