"""Authentication and form-rendering endpoints -- IDENTICAL between vuln/ and safe/.

The /form endpoint renders an HTML form that includes the session token as a
hidden field.  Whether this constitutes a vulnerability depends on whether
session.py's require_auth also accepts the token from form POST data.

Chain: session token in hidden field -> XSS reads DOM -> token replayed (CWE-384)
"""
from flask import Flask, request, jsonify, make_response
from session import create_session

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_a", "role": "admin"},
    "bob":   {"password": "pw_b", "role": "user"},
}


# vuln-code-snippet start chain_hidden_field_safe
@app.route("/login", methods=["POST"])
def login():
    """Authenticate and return a session token embedded in the response."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    token = create_session(username, user["role"])

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie("session_token", token, httponly=True)
    return resp


@app.route("/form")
def form():
    """Render a form without exposing the session token in the DOM.

    SAFE: the session token is carried only in the HttpOnly cookie and is
    not embedded in the HTML, so XSS cannot extract it from the page source.
    """
    html = (
        "<form method='POST' action='/action'>"  # vuln-code-snippet safe-line chain_hidden_field_safe
        "<input type='submit' value='Submit'>"
        "</form>"
    )
# vuln-code-snippet end chain_hidden_field_safe
    return html, 200, {"Content-Type": "text/html"}


@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session_token")
    return resp
