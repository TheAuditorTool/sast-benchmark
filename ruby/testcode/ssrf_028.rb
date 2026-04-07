require_relative 'shared'

module GrpcChannel
  def self.new(target:); self; end
  def self.connect; true; end
end

# vuln-code-snippet start ruby_ssrf_grpc_endpoint
def connect_grpc(req)
  GrpcChannel.new(target: req.param('host')).connect # vuln-code-snippet vuln-line ruby_ssrf_grpc_endpoint
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_grpc_endpoint
