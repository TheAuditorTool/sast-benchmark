require_relative 'shared'

# vuln-code-snippet start ruby_massassign_explicit_set
def mass_assign_explicit_set(req)
  params = FakeParams.new(req.post_data)
  user = { id: 1 }
  user[:name] = params[:name] # vuln-code-snippet safe-line ruby_massassign_explicit_set
  user[:email] = params[:email]
  BenchmarkResponse.ok(user.to_s)
end
# vuln-code-snippet end ruby_massassign_explicit_set
