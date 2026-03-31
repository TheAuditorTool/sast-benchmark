require_relative 'shared'

module FakeRedis
  def self.new(options = {})
    options
  end
end

# vuln-code-snippet start ruby_hardcoded_env_redis
def connect_to_cache(req)
  key = req.param('key')
  redis = FakeRedis.new(host: 'redis.internal', port: 6379, password: ENV['REDIS_PASSWORD'])  # vuln-code-snippet safe-line ruby_hardcoded_env_redis
  BenchmarkResponse.ok(redis.to_s)
end
# vuln-code-snippet end ruby_hardcoded_env_redis
