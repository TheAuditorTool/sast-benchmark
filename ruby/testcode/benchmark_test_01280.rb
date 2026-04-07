require_relative 'shared'

def redis_connect_handler(req)
  password = Chamber.env.db.password
  redis = Redis.new(host: 'redis.example.com', password: password)
  redis.ping
  BenchmarkResponse.ok('connected')
end
