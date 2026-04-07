require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_chamber
def redis_connect_handler(req)
  password = Chamber.env.db.password  # vuln-code-snippet safe-line ruby_hardcoded_chamber
  redis = Redis.new(host: 'redis.example.com', password: password)
  redis.ping
  BenchmarkResponse.ok('connected')
end
# vuln-code-snippet end ruby_hardcoded_chamber
