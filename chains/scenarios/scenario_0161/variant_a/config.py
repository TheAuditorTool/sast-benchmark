import os

IDP_ENTITY_ID = "https://idp.example.com"
SP_ACS_URL = "https://app.example.com/saml/acs"
SESSION_SECRET = os.environ.get("SESSION_SECRET", "saml-session-secret-placeholder")
SHADOW_FILE = "/etc/shadow"
