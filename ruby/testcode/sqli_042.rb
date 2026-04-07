require_relative 'shared'

# vuln-code-snippet start ruby_sqli_select_fixed
def export_user_profile(req)
  user_id = req.param('user_id').to_i   # tainted -- naive tool may flag this scope
  user = User.select(:id, :email, :name).find(user_id)  # vuln-code-snippet safe-line ruby_sqli_select_fixed
  BenchmarkResponse.json({ user: user&.to_h })
end
# vuln-code-snippet end ruby_sqli_select_fixed
