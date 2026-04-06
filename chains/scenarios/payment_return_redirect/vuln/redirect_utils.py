"""Payment return URL validation -- VULNERABLE variant.

is_safe_return_url() always returns True.  An attacker crafts a checkout URL
with a return_url pointing to a phishing page that mimics the payment success screen.

Chain: POST /checkout {return_url: https://evil.com} -> payment processed
       -> GET /payment/return?session=X -> redirect to phishing page
"""


def is_safe_return_url(url):
    """Validate payment return URL.

    BUG: always returns True.
    """
    return True  # VULN: any URL accepted
