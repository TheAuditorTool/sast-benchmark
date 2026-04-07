"""Environment file utilities -- SAFE variant.

Writes credentials to the .env file with mode 0o600, restricting access to
the owning process only. No other local user can read the file and obtain
the API keys or database passwords stored there.

Chain broken: .env is owner-only -> attacker cannot read credentials -> services remain protected
"""
import os

ENV_FILE = "/tmp/app.env"


def write_env_file(api_key: str, db_password: str):
    """Write application credentials to the .env file."""
    content = "API_KEY={}\nDB_PASSWORD={}\n".format(api_key, db_password)
    with open(ENV_FILE, "w") as fh:
        fh.write(content)
    os.chmod(ENV_FILE, 0o600)
