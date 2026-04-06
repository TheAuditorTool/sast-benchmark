"""User account service -- IDENTICAL between vuln/ and safe/ variants.

Flask application providing user registration and password reset. The two
modules interact: a weak email uniqueness check in registration.py allows
creating a second account for an email address that already has one, and
the password reset flow in password_reset.py sends a reset token to any
registered address -- creating an account takeover path.

Chain: email normalization bypass -> duplicate registration -> password reset
       token delivered to attacker -> victim's password overwritten -> account takeover
Individual findings: none in this file
Chain finding: registration bypass enables account takeover (critical, CWE-287)
"""
from flask import Flask
from registration import reg_bp
from password_reset import reset_bp

app = Flask(__name__)
app.register_blueprint(reg_bp)
app.register_blueprint(reset_bp)

# Shared in-memory user store accessed by both blueprints
import registration
import password_reset
password_reset.USER_STORE = registration.USER_STORE


@app.route("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)
