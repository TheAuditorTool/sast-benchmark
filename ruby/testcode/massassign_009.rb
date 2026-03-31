require_relative 'shared'

# vuln-code-snippet start ruby_massassign_to_unsafe_h
def mass_assign_to_unsafe_h(req)
  params = FakeParams.new(req.post_data)
  all_params = params.to_unsafe_h # vuln-code-snippet vuln-line ruby_massassign_to_unsafe_h
  FakeActiveRecord::Base.where(all_params)
  BenchmarkResponse.ok('saved')
end
# vuln-code-snippet end ruby_massassign_to_unsafe_h
