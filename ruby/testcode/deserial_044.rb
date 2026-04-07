require_relative 'shared'

# vuln-code-snippet start ruby_deser_msgpack_v2_typed
def msgpack_v2_typed_handler(req)
  factory = MessagePack::Factory.new
  factory.register_type(0x01, String)
  factory.register_type(0x02, Integer)
  obj = factory.unpacker.feed_each(req.post('data')).first  # vuln-code-snippet safe-line ruby_deser_msgpack_v2_typed
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_msgpack_v2_typed
