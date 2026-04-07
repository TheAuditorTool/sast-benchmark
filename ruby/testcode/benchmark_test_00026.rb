require_relative 'shared'

def parse_proto_instead_handler(req)
  data = req.post('data')
  msg = MyProtoMessage.decode(data)
  BenchmarkResponse.json({ id: msg.id })
end
