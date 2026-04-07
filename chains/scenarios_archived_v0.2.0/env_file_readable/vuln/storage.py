"""Environment file utilities -- VULNERABLE variant.

Writes application secrets (API keys, DB credentials) to a .env file with
mode 0o644, making it world-readable. Any local user can read the file and
obtain all credentials stored there, enabling lateral movement or direct
service abuse.

Chain: world-readable .env file written -> any user reads secrets -> credentials used to access services
Individual findings: secrets stored in world-readable .env (CWE-732)
Chain finding: credential theft via world-readable .env file (critical)
"""
import os

ENV_FILE = "/tmp/app.env"


def write_env_file(api_key: str, db_password: str):
    """Write application credentials to the .env file."""
    content = "API_KEY={}\nDB_PASSWORD={}\n".format(api_key, db_password)
    with open(ENV_FILE, "w") as fh:
        fh.write(content)
    os.chmod(ENV_FILE, 0o644)
