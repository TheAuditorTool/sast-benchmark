from flask import Blueprint, request, jsonify
from crypto import sign, verify

auth_bp = Blueprint("auth", __name__)

@auth_bp.route("/sign", methods=["POST"])
def do_sign():
    message = (request.json or {}).get("message", "")
    return jsonify({"message": message, "signature": sign(message)})

# vuln-code-snippet start ChainScenario0158B
@auth_bp.route("/verify")
def do_verify():
    message = request.args.get("message", "")
    signature = request.args.get("signature", "")
    if not verify(message, signature):
        return jsonify({"error": "Invalid signature"}), 403
    return jsonify({"status": "verified", "message": message})  # vuln-code-snippet target-line ChainScenario0158B
# vuln-code-snippet end ChainScenario0158B
