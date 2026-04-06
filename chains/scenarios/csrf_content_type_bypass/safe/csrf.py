"""CSRF protection helpers -- VULNERABLE variant.

The endpoint only checks Content-Type for application/json, assuming
JSON-only APIs are not susceptible to CSRF because browsers cannot send
JSON bodies from forms.  However Flask's request.get_json() has force=True
semantics when content_type is application/x-www-form-urlencoded -- it
returns None -- so the route falls back to reading request.form instead.
An attacker can submit a standard HTML form with the same field names.

This file is IDENTICAL between vuln/ and safe/ variants.
The vulnerability is in routes.py: accepting form data as a fallback.
"""
