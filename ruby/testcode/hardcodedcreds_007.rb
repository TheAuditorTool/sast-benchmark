require_relative 'shared'

module FakeRedis
  def self.new(options = {})
    options
  end
end

# vuln-code-snippet start ruby_hardcoded_redis_pass
def connect_to_cache(req)
  key = req.param('key')
  redis = FakeRedis.new(host: 'redis.internal', port: 6379, password: 'redis_pass_hardcoded_123')  # vuln-code-snippet vuln-line ruby_hardcoded_redis_pass
  BenchmarkResponse.ok(redis.to_s)
end
# vuln-code-snippet end ruby_hardcoded_redis_pass
