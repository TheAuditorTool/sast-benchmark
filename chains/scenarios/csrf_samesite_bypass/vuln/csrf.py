"""CSRF protection helpers -- VULNERABLE variant.

The application sets SameSite=Lax on session cookies and relies on this alone.
However the state-changing operation is on a GET endpoint, and SameSite=Lax
permits cookies on top-level GET navigations from cross-site pages.

This file is IDENTICAL between vuln/ and safe/ variants.
The fix is in routes.py: switching the endpoint to POST-only.
"""
