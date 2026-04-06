"""Application configuration for test_creds_in_prod scenario -- VULNERABLE variant.

Test credentials were left in place and are active in the production
deployment. An attacker can log in with the well-known test account
and gain access to any features it has been granted.

Chain: hardcoded TEST_USERNAME/TEST_PASSWORD -> auth.py login -> unauthorized access
Individual findings: test credentials active in production (critical)
Chain finding: unauthorized access via test credentials left in production (critical)
"""

TEST_USERNAME = "testuser"
TEST_PASSWORD = "test1234"
SECRET_KEY = "dev-only-secret"
