require_relative 'shared'

# vuln-code-snippet start ruby_massassign_require_permit
def mass_assign_require_permit(req)
  params = FakeParams.new(req.post_data)
  user_params = params.require(:user).permit(:name) # vuln-code-snippet safe-line ruby_massassign_require_permit
  FakeActiveRecord::Base.where(user_params.to_h)
  BenchmarkResponse.ok('saved')
end
# vuln-code-snippet end ruby_massassign_require_permit
