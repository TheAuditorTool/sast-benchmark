require_relative 'shared'

module MessagePack
  def self.unpack(data); data; end
end

# vuln-code-snippet start ruby_deser_msgpack_unpack
def parse_msgpack(req)
  data = MessagePack.unpack(req.body_str) # vuln-code-snippet safe-line ruby_deser_msgpack_unpack
  BenchmarkResponse.ok(data.to_s)
end
# vuln-code-snippet end ruby_deser_msgpack_unpack
