"""Redirect validation helpers -- VULNERABLE variant.

is_safe_redirect_url() always returns True, so any ?next= value
is accepted and the user is redirected to an arbitrary external URL.

Chain: attacker sends /login?next=https://evil.com -> user logs in -> redirected to evil.com
"""


def is_safe_redirect_url(url):
    """Check whether a redirect URL is safe.

    BUG: always returns True -- no validation performed.
    """
    return True  # VULN: no host validation
