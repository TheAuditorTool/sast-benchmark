import base64
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify, session

saml_bp = Blueprint("saml", __name__)

_SAML_ASSERT_NS = "urn:oasis:names:tc:SAML:2.0:assertion"

@saml_bp.route("/saml/acs", methods=["POST"])
def acs():
    encoded = request.form.get("SAMLResponse", "")
    if not encoded:
        return jsonify({"error": "missing SAMLResponse"}), 400
    try:
        xml_bytes = base64.b64decode(encoded)
    except Exception:
        return jsonify({"error": "invalid base64"}), 400
    try:
        root = ET.fromstring(xml_bytes)
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 400
    ns = _SAML_ASSERT_NS
    attrs = {
        a.get("Name"): a.findtext(f"{{{ns}}}AttributeValue")
        for a in root.iter(f"{{{ns}}}Attribute")
    }
    uid = attrs.get("uid") or attrs.get("NameID") or ""
    if not uid:
        return jsonify({"error": "no subject in assertion"}), 401
    session["uid"] = uid
    return jsonify({"authenticated": True, "uid": uid})
