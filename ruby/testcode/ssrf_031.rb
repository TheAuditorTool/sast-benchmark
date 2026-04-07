require_relative 'shared'

module RedisClient
  def self.new(url:); self; end
  def self.ping; true; end
end

# vuln-code-snippet start ruby_ssrf_redis_url
def ping_redis(req)
  RedisClient.new(url: req.param('redis_url')).ping # vuln-code-snippet vuln-line ruby_ssrf_redis_url
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_redis_url
