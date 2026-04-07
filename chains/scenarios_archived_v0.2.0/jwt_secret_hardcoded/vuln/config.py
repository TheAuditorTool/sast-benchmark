"""Application configuration for jwt_secret_hardcoded scenario -- VULNERABLE variant.

The JWT signing secret is hardcoded. An attacker can use it to forge
valid tokens for any user, including admins, without ever authenticating.

Chain: hardcoded JWT_SECRET -> auth.py token verification -> forged token accepted
Individual findings: hardcoded JWT secret (critical)
Chain finding: token forgery and unauthorized access via hardcoded JWT secret (critical)
"""

JWT_SECRET = "jwt-super-secret-key-do-not-share"
JWT_ALGORITHM = "HS256"
SECRET_KEY = "dev-only-secret"
