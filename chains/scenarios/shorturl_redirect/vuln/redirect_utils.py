"""Short-URL redirect validation -- VULNERABLE variant.

is_allowed_destination() always returns True, so any URL stored in the
shortener database is followed unconditionally.  An attacker can create
a short URL pointing to a phishing page.

Chain: POST /shorten {url: https://evil.com} -> GET /r/<code> -> redirect to phishing page
"""


def is_allowed_destination(url):
    """Validate that the destination URL is permissible.

    BUG: always returns True.
    """
    return True  # VULN: all destinations allowed
