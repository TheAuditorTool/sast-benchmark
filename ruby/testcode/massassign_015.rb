require_relative 'shared'

# vuln-code-snippet start ruby_massassign_create_raw
def create_raw_params(req)
  params = FakeParams.new(req.post('record') || {})
  BenchmarkResponse.ok("created: #{params.to_h}") # vuln-code-snippet vuln-line ruby_massassign_create_raw
end
# vuln-code-snippet end ruby_massassign_create_raw
