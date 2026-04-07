require_relative 'shared'

module GrpcChannel
  def self.new(target:); self; end
  def self.connect; true; end
end

def connect_grpc(req)
  GrpcChannel.new(target: req.param('host')).connect
  BenchmarkResponse.json({ ok: true })
end
