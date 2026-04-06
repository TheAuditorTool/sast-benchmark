"""Maven POM analyzer -- SAFE variant.

POST /build/analyze accepts a Maven pom.xml body and parses it with a
hardened XMLParser. Clearing parser.entity prevents external entity
declarations from being resolved, so crafted POMs that point to
/run/secrets/ci_token or other CI/CD secret paths produce no file read.

Chain broken: XMLParser with entity={} prevents entity resolution ->
CI secret exfiltration via POM upload is prevented
CWE-611: Improper Restriction of XML External Entity Reference
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

pom_bp = Blueprint("pom", __name__)

_MVN_NS = "http://maven.apache.org/POM/4.0.0"


def _safe_fromstring(text):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(text, parser=parser)


@pom_bp.route("/build/analyze", methods=["POST"])
def analyze_pom():
    """Accept a POM XML body and return extracted dependency coordinates."""
    raw = request.get_data()
    if len(raw) > config.MAX_POM_BYTES:
        return jsonify({"error": "POM too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_pom_xxe_safe
        root = _safe_fromstring(xml_text)  # vuln-code-snippet safe-line chain_pom_xxe_safe
# vuln-code-snippet end chain_pom_xxe_safe
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    deps = []
    for dep in root.iter("dependency"):
        g = dep.findtext("groupId") or dep.findtext(f"{{{_MVN_NS}}}groupId") or ""
        a = dep.findtext("artifactId") or dep.findtext(f"{{{_MVN_NS}}}artifactId") or ""
        v = dep.findtext("version") or dep.findtext(f"{{{_MVN_NS}}}version") or ""
        deps.append({"groupId": g, "artifactId": a, "version": v})
    return jsonify({"dependencies": deps})
