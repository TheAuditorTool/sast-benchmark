require_relative 'shared'

# vuln-code-snippet start ruby_massassign_strong_params
def mass_assign_strong_params(req)
  params = FakeParams.new(req.post_data)
  user_params = params.require(:user).permit(:name, :bio) # vuln-code-snippet safe-line ruby_massassign_strong_params
  FakeActiveRecord::Base.where(user_params.to_h)
  BenchmarkResponse.ok('saved')
end
# vuln-code-snippet end ruby_massassign_strong_params
