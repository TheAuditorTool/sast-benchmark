"""Maven POM analyzer -- VULNERABLE variant.

POST /build/analyze accepts a Maven pom.xml body and parses it with
xml.etree.ElementTree's default parser. Because external entities are
not restricted, a crafted POM can exfiltrate CI/CD credentials:

    <!DOCTYPE project [<!ENTITY ci SYSTEM "file:///run/secrets/ci_token">]>
    <project>
      <dependencies>
        <dependency><groupId>&ci;</groupId></dependency>
      </dependencies>
    </project>

The resolved entity (the CI token) appears in the extracted groupId.

Chain: POM body -> ET.fromstring (entities enabled) -> CI secret file read
Individual findings: XXE via POM upload (high)
Chain finding: XXE enables reading CI/CD secrets via build config upload (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

pom_bp = Blueprint("pom", __name__)

_MVN_NS = "http://maven.apache.org/POM/4.0.0"


@pom_bp.route("/build/analyze", methods=["POST"])
def analyze_pom():
    """Accept a POM XML body and return extracted dependency coordinates."""
    raw = request.get_data()
    if len(raw) > config.MAX_POM_BYTES:
        return jsonify({"error": "POM too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_pom_xxe_vuln
        root = ET.fromstring(xml_text)  # vuln-code-snippet vuln-line chain_pom_xxe_vuln
# vuln-code-snippet end chain_pom_xxe_vuln
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    deps = []
    for dep in root.iter("dependency"):
        g = dep.findtext("groupId") or dep.findtext(f"{{{_MVN_NS}}}groupId") or ""
        a = dep.findtext("artifactId") or dep.findtext(f"{{{_MVN_NS}}}artifactId") or ""
        v = dep.findtext("version") or dep.findtext(f"{{{_MVN_NS}}}version") or ""
        deps.append({"groupId": g, "artifactId": a, "version": v})
    return jsonify({"dependencies": deps})
