require_relative 'shared'

def redis_marshal_deserialize_handler(req)
  key = req.param('key')
  raw = $redis.get(key)
  obj = Marshal.load(raw)
  BenchmarkResponse.json({ result: obj.to_s })
end
