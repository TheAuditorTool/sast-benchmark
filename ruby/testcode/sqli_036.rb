require_relative 'shared'

# vuln-code-snippet start ruby_sqli_select_raw
def export_user_fields(req)
  extra_col = req.param('extra')
  users = User.select(Arel.sql("id, email, #{extra_col}"))  # vuln-code-snippet vuln-line ruby_sqli_select_raw
  BenchmarkResponse.json({ users: users.map { |u| u.to_h } })
end
# vuln-code-snippet end ruby_sqli_select_raw
