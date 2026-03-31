require_relative 'shared'

# vuln-code-snippet start ruby_massassign_permit_bang
def mass_assign_permit_bang(req)
  params = FakeParams.new(req.post_data)
  params.permit! # vuln-code-snippet vuln-line ruby_massassign_permit_bang
  user = FakeActiveRecord::Base.where(params.to_h)
  BenchmarkResponse.ok(user.to_a.first.to_s)
end
# vuln-code-snippet end ruby_massassign_permit_bang
