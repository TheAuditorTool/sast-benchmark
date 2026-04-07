require_relative 'shared'

module MessagePack
  def self.unpack(raw)
    JSON.parse(raw)
  end
end

def receive_message(req)
  raw = req.body_str
  payload = MessagePack.unpack(raw)
  msg = payload.fetch('text', '').to_s
  BenchmarkResponse.ok("received: #{msg}")
end
