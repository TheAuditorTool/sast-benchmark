require_relative 'shared'

# vuln-code-snippet start ruby_massassign_new_params
def mass_assign_new_params(req)
  result = FakeActiveRecord::Base.where(req.post_data) # vuln-code-snippet vuln-line ruby_massassign_new_params
  BenchmarkResponse.ok(result.to_a.first.to_s)
end
# vuln-code-snippet end ruby_massassign_new_params
