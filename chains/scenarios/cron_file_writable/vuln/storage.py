"""Cron configuration utilities -- VULNERABLE variant.

Writes the application cron job definition to a file with mode 0o666.
Any local user can modify this file to inject arbitrary cron entries,
which the cron daemon will execute with the application user's privileges.

Chain: world-writable cron file written -> attacker injects cron entry -> cron executes attacker command
Individual findings: insecure cron file permissions (CWE-732)
Chain finding: scheduled command injection via world-writable cron file (critical)
"""
import os

CRON_FILE = "/tmp/app_cron.txt"

DEFAULT_SCHEDULE = "*/5 * * * * /usr/bin/app-healthcheck\n"


def write_cron_config():
    """Write the default cron schedule to the cron config file."""
    with open(CRON_FILE, "w") as fh:
        fh.write(DEFAULT_SCHEDULE)
    os.chmod(CRON_FILE, 0o666)
