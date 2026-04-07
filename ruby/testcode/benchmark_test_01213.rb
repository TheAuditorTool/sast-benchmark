require_relative 'shared'

module RedisClient
  def self.new(url:); self; end
  def self.ping; true; end
end

def ping_redis(req)
  RedisClient.new(url: req.param('redis_url')).ping
  BenchmarkResponse.json({ ok: true })
end
