require_relative 'shared'

# vuln-code-snippet start ruby_deser_protobuf
def protobuf_deserialize_handler(req)
  data = req.post('data')
  msg = MyProtoMessage.decode(data)  # vuln-code-snippet safe-line ruby_deser_protobuf # Uses google-protobuf gem -- schema-bound, no arbitrary object creation
  BenchmarkResponse.json({ result: msg.to_h })
end
# vuln-code-snippet end ruby_deser_protobuf
