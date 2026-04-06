"""User registration -- SAFE variant.

Accepts email and password and creates a new user account. Before the
uniqueness check, the email address is normalized: lowercased and stripped
of any plus-tag (the local part up to '+' is kept, the tag discarded).
This ensures that user+evil@example.com is treated as user@example.com and
rejected when that address is already registered.

Chain: POST /auth/register with victim+tag@domain -> normalize_email() strips
       tag -> normalized address matches existing victim account -> 409 returned
       -> duplicate account not created -> password reset cannot be triggered
Individual findings: none -- email normalized before uniqueness check
Chain finding: none -- duplicate registration prevented (CWE-287 mitigated)
"""
import re
from flask import Blueprint, request, jsonify

reg_bp = Blueprint("reg", __name__)

USER_STORE: dict[str, dict] = {}

_EMAIL_RE = re.compile(r'^[^@\s]+@[^@\s]+\.[^@\s]+$')


def normalize_email(email: str) -> str:
    """Normalize an email address for storage and lookup.

    Lowercases the entire address and strips plus-tags from the local part
    so that user+tag@Example.COM canonicalizes to user@example.com.
    """
    email = email.lower()
    local, _, domain = email.partition("@")
    local = local.split("+")[0]
    return f"{local}@{domain}"


# vuln-code-snippet start chain_signup_takeover_safe
@reg_bp.route("/auth/register", methods=["POST"])
def register():
    """Register a new user account.

    Expects JSON with 'email' and 'password'. The email is normalized
    (lowercased, plus-tags stripped) before the uniqueness check.
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

    canonical = normalize_email(email)
    if canonical in USER_STORE:  # vuln-code-snippet safe-line chain_signup_takeover_safe
        return jsonify({"error": "Email already registered"}), 409

    USER_STORE[canonical] = {"email": canonical, "password": password}
    return jsonify({"status": "registered", "email": canonical}), 201
# vuln-code-snippet end chain_signup_takeover_safe
