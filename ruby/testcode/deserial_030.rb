require_relative 'shared'

# vuln-code-snippet start ruby_deser_msgpack_ext_type
def msgpack_ext_type_handler(req)
  MessagePack::DefaultFactory.register_type(0x00, String, packer: ->(v) { v }, unpacker: ->(v) { eval(v) })
  obj = MessagePack.unpack(req.post('data'))  # vuln-code-snippet vuln-line ruby_deser_msgpack_ext_type
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_msgpack_ext_type
