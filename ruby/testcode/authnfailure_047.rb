require_relative 'shared'

RATE_LIMIT_MAX = 10
RATE_LIMIT_WINDOW = 60

class FakeRedis
  def initialize; @store = {}; end
  def incr(key); @store[key] = (@store[key] || 0) + 1; end
  def expire(key, seconds); end
  def get(key); @store[key] || 0; end
end

# vuln-code-snippet start ruby_authn_rate_limit_redis
def login_with_rate_limit(req, redis)
  ip = req.header('REMOTE_ADDR')
  rate_key = "login_attempts:#{ip}"
  attempts = redis.incr(rate_key)
  redis.expire(rate_key, RATE_LIMIT_WINDOW)
  return BenchmarkResponse.error('Too many requests', 429) if attempts > RATE_LIMIT_MAX # vuln-code-snippet safe-line ruby_authn_rate_limit_redis
  username = req.post('username')
  password = req.post('password')
  BenchmarkResponse.ok("Login attempt for #{username}")
end
# vuln-code-snippet end ruby_authn_rate_limit_redis
