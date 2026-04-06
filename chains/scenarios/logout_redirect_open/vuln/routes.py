"""Route definitions for logout open redirect scenario -- VULNERABLE variant.

GET /logout clears the session and redirects to ?redirect=.  An attacker can
embed a link like /logout?redirect=https://evil.com in a phishing email.
The user's session is cleared (reducing suspicion) and they land on a
fake login page that captures credentials.

Chain: user clicks /logout?redirect=https://evil.com -> session cleared -> sent to phishing page
Individual findings: unvalidated redirect on logout (medium)
Chain finding: open redirect on logout -> credential phishing (high)
"""
from flask import request, redirect, session
from app import app
from redirect_utils import is_safe_redirect_url


@app.route("/logout")
def logout():
    """Clear the session and redirect."""
    session.clear()
    redirect_url = request.args.get("redirect", "/")

# vuln-code-snippet start chain_logout_redirect_vuln
    if not is_safe_redirect_url(redirect_url):
        redirect_url = "/"
    return redirect(redirect_url)  # vuln-code-snippet vuln-line chain_logout_redirect_vuln
# vuln-code-snippet end chain_logout_redirect_vuln


if __name__ == "__main__":
    app.run(port=5000)
