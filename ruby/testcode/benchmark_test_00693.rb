require_relative 'shared'

def msgpack_type_deserialize_handler(req)
  MessagePack::DefaultFactory.register_type(0x01, Object)
  obj = MessagePack.unpack(req.post('data'))
  BenchmarkResponse.json({ result: obj.to_s })
end
