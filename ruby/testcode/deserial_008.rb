require_relative 'shared'

module MessagePack
  def self.unpack(raw)
    JSON.parse(raw)
  end
end

# vuln-code-snippet start ruby_deser_msgpack
def receive_message(req)
  raw = req.body_str
  payload = MessagePack.unpack(raw) # vuln-code-snippet safe-line ruby_deser_msgpack
  msg = payload.fetch('text', '').to_s
  BenchmarkResponse.ok("received: #{msg}")
end
# vuln-code-snippet end ruby_deser_msgpack
