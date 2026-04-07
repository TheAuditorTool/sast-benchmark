require_relative 'shared'

# vuln-code-snippet start ruby_deser_msgpack_type
def msgpack_type_deserialize_handler(req)
  MessagePack::DefaultFactory.register_type(0x01, Object)
  obj = MessagePack.unpack(req.post('data'))  # vuln-code-snippet vuln-line ruby_deser_msgpack_type
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_msgpack_type
