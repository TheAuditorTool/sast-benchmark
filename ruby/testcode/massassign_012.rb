require_relative 'shared'

# vuln-code-snippet start ruby_massassign_permit_specific
def update_user_safe(req)
  params = FakeParams.new(req.post('user') || {})
  safe_attrs = params.permit(:name, :email) # vuln-code-snippet safe-line ruby_massassign_permit_specific
  BenchmarkResponse.ok("updated: #{safe_attrs.to_h}")
end
# vuln-code-snippet end ruby_massassign_permit_specific
