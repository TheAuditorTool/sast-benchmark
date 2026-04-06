"""Route definitions for logout open redirect scenario -- SAFE variant.

GET /logout validates the ?redirect= parameter and falls back to / for external URLs.

Chain: user clicks /logout?redirect=https://evil.com -> is_safe_redirect_url rejects -> /
Individual findings: none
Chain finding: none -- redirect is validated
"""
from flask import request, redirect, session
from app import app
from redirect_utils import is_safe_redirect_url


@app.route("/logout")
def logout():
    """Clear the session and redirect."""
    session.clear()
    redirect_url = request.args.get("redirect", "/")

# vuln-code-snippet start chain_logout_redirect_safe
    if not is_safe_redirect_url(redirect_url):
        redirect_url = "/"
    return redirect(redirect_url)  # vuln-code-snippet safe-line chain_logout_redirect_safe
# vuln-code-snippet end chain_logout_redirect_safe


if __name__ == "__main__":
    app.run(port=5000)
