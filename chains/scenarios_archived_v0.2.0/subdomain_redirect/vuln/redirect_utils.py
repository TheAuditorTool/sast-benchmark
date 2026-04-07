"""Subdomain redirect validation -- VULNERABLE variant.

is_safe_redirect_url() uses re.match with .*\\.example\\.com, which matches
hosts like evil-example.com because .* can match "evil-".  An attacker
can bypass this check by registering evil-example.com.

Chain: /go?url=https://evil-example.com/phish -> regex matches -> redirect to phishing page
"""
import re

SUBDOMAIN_PATTERN = re.compile(r"https?://.*\.example\.com(/.*)?$")


def is_safe_redirect_url(url):
    """Check that the URL belongs to example.com.

    BUG: the pattern matches evil-example.com because .* is too greedy.
    """
    return bool(SUBDOMAIN_PATTERN.match(url))  # VULN: evil-example.com matches
