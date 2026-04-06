"""Application configuration for api_key_in_config scenario -- VULNERABLE variant.

The external service API key is hardcoded. An attacker who reads this
file can make authenticated calls to the external service as this application.

Chain: hardcoded API_KEY -> auth.py request -> unauthorized API access
Individual findings: hardcoded API key (critical)
Chain finding: unauthorized external API access via hardcoded key (critical)
"""

API_KEY = "sk-live-4f8a2b1c9d3e7f6a0b5c"
API_ENDPOINT = "https://api.example.com/v1"
SECRET_KEY = "dev-only-secret"
