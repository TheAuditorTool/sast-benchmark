require_relative 'shared'

def thrift_typed_deserialize_handler(req)
  buf = req.post('data')
  obj = MyThriftStruct.new
  Thrift::Deserializer.new.deserialize(obj, buf)
  BenchmarkResponse.json({ result: obj.to_s })
end
