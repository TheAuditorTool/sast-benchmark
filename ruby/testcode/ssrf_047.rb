require_relative 'shared'

module GrpcChannel
  def self.new(target:); self; end
  def self.connect; true; end
end

GRPC_HOST = ENV.fetch('GRPC_HOST').freeze rescue 'grpc.internal.example.com'.freeze

# vuln-code-snippet start ruby_ssrf_grpc_constant_host
def connect_grpc_fixed(req)
  GrpcChannel.new(target: GRPC_HOST).connect # vuln-code-snippet safe-line ruby_ssrf_grpc_constant_host
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_grpc_constant_host
