require_relative 'shared'

# vuln-code-snippet start ruby_xxe_proto_instead
def parse_proto_instead_handler(req)
  data = req.post('data')
  msg = MyProtoMessage.decode(data) # vuln-code-snippet safe-line ruby_xxe_proto_instead
  BenchmarkResponse.json({ id: msg.id })
end
# vuln-code-snippet end ruby_xxe_proto_instead
