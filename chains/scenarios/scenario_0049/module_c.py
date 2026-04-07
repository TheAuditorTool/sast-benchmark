import os

ENV_FILE = "/tmp/app.env"

def write_env_file(api_key: str, db_password: str):
    content = "API_KEY={}\nDB_PASSWORD={}\n".format(api_key, db_password)
    with open(ENV_FILE, "w") as fh:
        fh.write(content)
    os.chmod(ENV_FILE, 0o600)
