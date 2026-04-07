"""SAML ACS handler -- SAFE variant.

POST /saml/acs decodes the SAMLResponse form field (base64) and parses
the XML with a hardened XMLParser. Clearing parser.entity ensures that
any external entity declarations in the SAML Response XML are ignored.
File URIs such as file:///etc/shadow are never resolved, so their
contents cannot be exfiltrated via attribute values.

Chain broken: XMLParser with entity={} blocks external entity resolution ->
/etc/shadow read is prevented
CWE-611: Improper Restriction of XML External Entity Reference
"""
import base64
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify, session

saml_bp = Blueprint("saml", __name__)

_SAML_ASSERT_NS = "urn:oasis:names:tc:SAML:2.0:assertion"


def _safe_fromstring(data):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(data, parser=parser)


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
# vuln-code-snippet start chain_saml_xxe_safe
        root = _safe_fromstring(xml_bytes)  # vuln-code-snippet safe-line chain_saml_xxe_safe
# vuln-code-snippet end chain_saml_xxe_safe
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
