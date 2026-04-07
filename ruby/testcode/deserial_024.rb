require_relative 'shared'

# vuln-code-snippet start ruby_deser_redis_marshal
def redis_marshal_deserialize_handler(req)
  key = req.param('key')
  raw = $redis.get(key)
  obj = Marshal.load(raw)  # vuln-code-snippet vuln-line ruby_deser_redis_marshal
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_redis_marshal
