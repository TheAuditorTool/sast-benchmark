import os

SECRET_KEY = os.environ.get("SECRET_KEY", "dev-secret-do-not-use-in-prod")
DEBUG = os.environ.get("DEBUG", "true").lower() == "true"

SESSION_STORE = {
    "tok-alice-abc123": "u1",
    "tok-admin-xyz999": "admin",
}
