require_relative 'shared'

def msgpack_restricted_handler(req)
  data = req.post('data')
  obj = MessagePack.unpack(data)
  BenchmarkResponse.json({ result: obj })
end
