require_relative 'shared'

# vuln-code-snippet start ruby_massassign_assign_attrs
def mass_assign_assign_attrs(req)
  user = { id: 1, role: 'user' }
  user.merge!(req.post_data) # vuln-code-snippet vuln-line ruby_massassign_assign_attrs
  BenchmarkResponse.ok(user.to_s)
end
# vuln-code-snippet end ruby_massassign_assign_attrs
