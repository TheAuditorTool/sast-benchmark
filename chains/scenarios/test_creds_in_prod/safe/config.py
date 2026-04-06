"""Application configuration for test_creds_in_prod scenario -- SAFE variant.

Test credentials have been removed. There is no hardcoded bypass path
in config; authentication must go through the standard user store.

Chain broken: no TEST_USERNAME/TEST_PASSWORD defined -> no bypass login path
"""

SECRET_KEY = "dev-only-secret"
