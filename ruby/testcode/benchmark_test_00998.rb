require_relative 'shared'

module FakeRedis
  def self.new(options = {})
    options
  end
end

def connect_to_cache(req)
  key = req.param('key')
  redis = FakeRedis.new(host: 'redis.internal', port: 6379, password: ENV['REDIS_PASSWORD'])
  BenchmarkResponse.ok(redis.to_s)
end
