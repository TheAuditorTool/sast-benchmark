require_relative 'shared'

# vuln-code-snippet start ruby_massassign_update_unsafe_h
def update_user_unsafe(req)
  params = FakeParams.new(req.post('user') || {})
  attrs = params.to_unsafe_h
  BenchmarkResponse.ok("updated: #{attrs}") # vuln-code-snippet vuln-line ruby_massassign_update_unsafe_h
end
# vuln-code-snippet end ruby_massassign_update_unsafe_h
