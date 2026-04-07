"""Redirect validation for email verification -- VULNERABLE variant.

is_safe_redirect_url() always returns True.  After clicking the verification
link in a phishing email, the user is redirected to any attacker-supplied URL.

Chain: GET /verify?token=X&next=https://evil.com -> verified -> redirect to phishing page
"""


def is_safe_redirect_url(url):
    """Check whether the post-verification redirect URL is safe.

    BUG: always returns True.
    """
    return True  # VULN: no host check
