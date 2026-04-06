"""User registration -- VULNERABLE variant.

Accepts email and password and creates a new user account. The uniqueness
check uses the raw email address as the lookup key without any normalization.
Email addresses are canonically case-insensitive and many providers ignore
plus-tags (user+anything@example.com delivers to user@example.com). By not
normalizing, an attacker can register user+evil@example.com when the victim
account user@example.com already exists, creating a second account that
shares the effective delivery address.

Once both accounts exist, the password reset flow (password_reset.py) will
issue a token to whichever address the attacker controls, enabling them to
reset the victim's effective email password.

Chain: POST /auth/register with victim+tag@domain -> email not normalized ->
       duplicate account created -> POST /auth/password-reset/request ->
       token issued to attacker email -> victim account takeover
Individual findings: missing email normalization allows duplicate registration (high)
Chain finding: registration bypass enables account takeover via password reset
               (critical, CWE-287)
"""
import re
from flask import Blueprint, request, jsonify

reg_bp = Blueprint("reg", __name__)

USER_STORE: dict[str, dict] = {}

_EMAIL_RE = re.compile(r'^[^@\s]+@[^@\s]+\.[^@\s]+$')


# vuln-code-snippet start chain_signup_takeover_vuln
@reg_bp.route("/auth/register", methods=["POST"])
def register():
    """Register a new user account.

    Expects JSON with 'email' and 'password'. The email is stored as-is,
    without normalizing case or stripping plus-tags.
    """
    body = request.get_json(silent=True)
    if not body:
        return jsonify({"error": "JSON body required"}), 400

    email = body.get("email", "").strip()
    password = body.get("password", "")

    if not email or not _EMAIL_RE.match(email):
        return jsonify({"error": "Valid email address required"}), 400
    if not password or len(password) < 8:
        return jsonify({"error": "Password must be at least 8 characters"}), 400

    # No normalization -- allows user+tag@example.com alongside user@example.com
    if email in USER_STORE:  # vuln-code-snippet vuln-line chain_signup_takeover_vuln
        return jsonify({"error": "Email already registered"}), 409

    USER_STORE[email] = {"email": email, "password": password}
    return jsonify({"status": "registered", "email": email}), 201
# vuln-code-snippet end chain_signup_takeover_vuln
