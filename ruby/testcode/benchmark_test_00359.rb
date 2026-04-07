require_relative 'shared'

def protobuf_deserialize_handler(req)
  data = req.post('data')
  msg = MyProtoMessage.decode(data)
  BenchmarkResponse.json({ result: msg.to_h })
end
