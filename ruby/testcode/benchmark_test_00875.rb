require_relative 'shared'

module MessagePack
  def self.unpack(data); data; end
end

def parse_msgpack(req)
  data = MessagePack.unpack(req.body_str)
  BenchmarkResponse.ok(data.to_s)
end
