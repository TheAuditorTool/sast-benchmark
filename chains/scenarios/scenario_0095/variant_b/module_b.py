import os

DAV_ROOT = "/var/dav"
PRIVATE_KEY_PATH = "/etc/ssl/private/server.key"
DAV_ADMIN_USER = os.environ.get("DAV_ADMIN_USER", "davadmin")
DAV_ADMIN_PASS = os.environ.get("DAV_ADMIN_PASS", "dav-admin-pass-placeholder")
MAX_PROPFIND_BYTES = 64 * 1024
