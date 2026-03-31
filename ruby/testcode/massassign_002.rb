require_relative 'shared'

# vuln-code-snippet start ruby_massassign_permit_list
def mass_assign_permit_list(req)
  params = FakeParams.new(req.post_data)
  allowed = params.permit(:name, :email) # vuln-code-snippet safe-line ruby_massassign_permit_list
  user = FakeActiveRecord::Base.where(allowed.to_h)
  BenchmarkResponse.ok(user.to_a.first.to_s)
end
# vuln-code-snippet end ruby_massassign_permit_list
