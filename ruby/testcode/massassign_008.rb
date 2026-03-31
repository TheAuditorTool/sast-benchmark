require_relative 'shared'

# vuln-code-snippet start ruby_massassign_slice
def mass_assign_slice(req)
  safe_data = req.post_data.slice('name', 'email') # vuln-code-snippet safe-line ruby_massassign_slice
  FakeActiveRecord::Base.where(safe_data)
  BenchmarkResponse.ok('saved')
end
# vuln-code-snippet end ruby_massassign_slice
