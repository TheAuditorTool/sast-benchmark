require_relative 'shared'

module GrpcChannel
  def self.new(target:); self; end
  def self.connect; true; end
end

GRPC_HOST = ENV.fetch('GRPC_HOST').freeze rescue 'grpc.internal.example.com'.freeze

def connect_grpc_fixed(req)
  GrpcChannel.new(target: GRPC_HOST).connect
  BenchmarkResponse.json({ ok: true })
end
