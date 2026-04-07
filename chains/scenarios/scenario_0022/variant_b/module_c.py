import os

CRON_FILE = "/tmp/app_cron.txt"

DEFAULT_SCHEDULE = "*/5 * * * * /usr/bin/app-healthcheck\n"

def write_cron_config():
    with open(CRON_FILE, "w") as fh:
        fh.write(DEFAULT_SCHEDULE)
    os.chmod(CRON_FILE, 0o600)
