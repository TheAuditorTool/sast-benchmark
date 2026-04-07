"""Configuration for the rss_feed_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

FEED_CACHE_DIR = "/tmp/rss_cache"
MAX_FEED_BYTES = 512 * 1024
INTERNAL_SECRET = os.environ.get("INTERNAL_SECRET", "rss-secret-placeholder")
