"""HTTP fetcher -- IDENTICAL between vuln/ and safe/.

Performs the actual HTTP fetch after the validator has approved the URL.
The key race condition is that this module calls urllib.request.urlopen,
which performs its own DNS resolution. If the attacker controls the DNS TTL
(TTL=0) and flips the A record between the validator's resolution and this
fetch, the check is bypassed.

This file is byte-identical between vuln/ and safe/ -- the vulnerability
is entirely in validator.py (how the IP is passed to this module).
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)


def perform_fetch(url):
    """Execute an HTTP GET for the approved URL and return the response body."""
    response = urllib.request.urlopen(url)
    return response.read(8192), response.headers.get("Content-Type", "application/octet-stream")
