"""SAML ACS handler -- VULNERABLE variant.

POST /saml/acs decodes the SAMLResponse form field (base64) and parses
the XML with xml.etree.ElementTree's default settings. External entity
declarations in the SAML Response XML are resolved before attribute
extraction, enabling an attacker to read /etc/shadow:

    <!DOCTYPE samlp:Response [
      <!ENTITY shadow SYSTEM "file:///etc/shadow">
    ]>
    <samlp:Response ...>
      <saml:Attribute Name="uid"><saml:AttributeValue>&shadow;</saml:AttributeValue></saml:Attribute>
    </samlp:Response>

Chain: SAML ACS POST -> base64 decode -> ET.fromstring (entities enabled) -> /etc/shadow read
Individual findings: XXE in SAML ACS endpoint (high)
Chain finding: XXE enables reading /etc/shadow via SAML SSO flow (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
import base64
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify, session

saml_bp = Blueprint("saml", __name__)

_SAML_ASSERT_NS = "urn:oasis:names:tc:SAML:2.0:assertion"


@saml_bp.route("/saml/acs", methods=["POST"])
def acs():
    """Assertion Consumer Service: decode and validate SAML Response."""
    encoded = request.form.get("SAMLResponse", "")
    if not encoded:
        return jsonify({"error": "missing SAMLResponse"}), 400
    try:
        xml_bytes = base64.b64decode(encoded)
    except Exception:
        return jsonify({"error": "invalid base64"}), 400
    try:
# vuln-code-snippet start chain_saml_xxe_vuln
        root = ET.fromstring(xml_bytes)  # vuln-code-snippet vuln-line chain_saml_xxe_vuln
# vuln-code-snippet end chain_saml_xxe_vuln
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
