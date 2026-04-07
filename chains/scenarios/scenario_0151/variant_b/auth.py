from flask import Blueprint, request, jsonify
from crypto import sign, verify

auth_bp = Blueprint("auth", __name__)

@auth_bp.route("/sign", methods=["POST"])
def do_sign():
    message = (request.json or {}).get("message", "")
    result = sign(message)
    return jsonify(result)

# vuln-code-snippet start ChainScenario0151B
@auth_bp.route("/verify")
def do_verify():
    message = request.args.get("message", "")
    try:
        r = int(request.args.get("r", "0"))
        s = int(request.args.get("s", "0"))
    except ValueError:
        return jsonify({"error": "Bad parameters"}), 400
    if not verify(message, r, s):
        return jsonify({"error": "Invalid signature"}), 403
    return jsonify({"status": "verified", "message": message})  # vuln-code-snippet target-line ChainScenario0151B
# vuln-code-snippet end ChainScenario0151B
