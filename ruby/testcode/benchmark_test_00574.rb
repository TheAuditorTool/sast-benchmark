require_relative 'shared'

def msgpack_ext_type_handler(req)
  MessagePack::DefaultFactory.register_type(0x00, String, packer: ->(v) { v }, unpacker: ->(v) { eval(v) })
  obj = MessagePack.unpack(req.post('data'))
  BenchmarkResponse.json({ result: obj.to_s })
end
