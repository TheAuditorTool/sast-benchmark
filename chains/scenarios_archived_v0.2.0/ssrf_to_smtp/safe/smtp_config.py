"""SMTP relay configuration -- IDENTICAL between vuln/ and safe/.

Represents an internal SMTP relay on smtp-relay.internal:25 that trusts all
connections from within the VPC. The relay does not require authentication
and will forward mail to any recipient, making it an open relay for anyone
who can reach it from the application server.

Chain: attacker -> /notify?template_url=http://smtp-relay.internal:25/... -> open relay

SMTP relay exposure:
  smtp-relay.internal:25 -- Postfix, no auth required from 10.0.0.0/8
    EHLO + MAIL FROM + RCPT TO + DATA -> send mail as any internal address
    Can be reached via SSRF using HTTP-to-SMTP cross-protocol attack
    Gopher protocol can be used to send raw SMTP commands via older urllib versions
"""

SMTP_RELAY_HOST = "smtp-relay.internal"
SMTP_RELAY_PORT = 25
SMTP_FROM_ADDRESS = "notifications@example.com"
SMTP_REQUIRE_AUTH = False
