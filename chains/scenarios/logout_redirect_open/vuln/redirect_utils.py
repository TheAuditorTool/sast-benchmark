"""Redirect validation helpers -- VULNERABLE variant.

is_safe_redirect_url() always returns True, so any ?redirect= value on
the logout endpoint is followed without host validation.

Chain: /logout?redirect=https://evil.com -> session cleared -> user sent to phishing page
"""


def is_safe_redirect_url(url):
    """Check whether a redirect URL is safe to follow after logout.

    BUG: always returns True.
    """
    return True  # VULN: no host validation
