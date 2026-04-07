require_relative 'shared'

def msgpack_v2_typed_handler(req)
  factory = MessagePack::Factory.new
  factory.register_type(0x01, String)
  factory.register_type(0x02, Integer)
  obj = factory.unpacker.feed_each(req.post('data')).first
  BenchmarkResponse.json({ result: obj.to_s })
end
