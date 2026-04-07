"""Cron configuration utilities -- SAFE variant.

Writes the cron job definition to a file with mode 0o600. Only the owning
process can read or modify the file. A local attacker cannot inject
additional cron entries, eliminating the scheduled command injection path.

Chain broken: cron file is owner-only -> no external modifications -> loader reads authentic schedule
"""
import os

CRON_FILE = "/tmp/app_cron.txt"

DEFAULT_SCHEDULE = "*/5 * * * * /usr/bin/app-healthcheck\n"


def write_cron_config():
    """Write the default cron schedule to the cron config file."""
    with open(CRON_FILE, "w") as fh:
        fh.write(DEFAULT_SCHEDULE)
    os.chmod(CRON_FILE, 0o600)
