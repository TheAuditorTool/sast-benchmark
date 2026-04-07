require_relative 'shared'

# vuln-code-snippet start ruby_deser_thrift_typed
def thrift_typed_deserialize_handler(req)
  buf = req.post('data')
  obj = MyThriftStruct.new
  Thrift::Deserializer.new.deserialize(obj, buf)  # vuln-code-snippet safe-line ruby_deser_thrift_typed
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_thrift_typed
