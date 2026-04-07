require_relative 'shared'

# vuln-code-snippet start ruby_deser_msgpack_restricted
def msgpack_restricted_handler(req)
  data = req.post('data')
  obj = MessagePack.unpack(data)  # vuln-code-snippet safe-line ruby_deser_msgpack_restricted # no custom types registered -- only primitive types
  BenchmarkResponse.json({ result: obj })
end
# vuln-code-snippet end ruby_deser_msgpack_restricted
