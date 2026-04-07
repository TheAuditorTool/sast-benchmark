"""Configuration for the xml_rpc_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

XMLRPC_MAX_BYTES = 128 * 1024
APP_CONFIG_PATH = "/etc/app/config.ini"
DATABASE_PASSWORD = os.environ.get("DATABASE_PASSWORD", "db-pass-placeholder")
RPC_AUTH_TOKEN = os.environ.get("RPC_AUTH_TOKEN", "rpc-auth-token-placeholder")
