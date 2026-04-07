"""Redis connection configuration -- IDENTICAL between vuln/ and safe/.

Represents an unauthenticated Redis instance on localhost:6379 used for
session caching. The Redis protocol on port 6379 accepts raw TCP commands
with no authentication, so any process (or proxied request) that can reach
localhost can read and overwrite arbitrary keys.

Chain: attacker -> /preview?url=http://localhost:6379/<RESP commands> -> Redis

Exposed attack surface:
  localhost:6379 -- Redis 7.x, no requirepass set, no bind restriction
    KEYS *           -> enumerate all cached session tokens
    GET session:<id> -> steal authenticated session
    SET session:<id> -> forge a session as any user
"""

REDIS_CONFIG = {
    "host": "localhost",
    "port": 6379,
    "db": 0,
    "password": None,
    "decode_responses": True,
}

SESSION_KEY_PREFIX = "session:"
CACHE_TTL_SECONDS = 3600
