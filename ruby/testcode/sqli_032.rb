require_relative 'shared'

# vuln-code-snippet start ruby_sqli_joins_string
def get_users_with_role(req)
  role_id = req.param('role_id')
  users = User.joins("LEFT JOIN roles ON roles.id = #{role_id}")  # vuln-code-snippet vuln-line ruby_sqli_joins_string
  BenchmarkResponse.json({ users: users.map { |u| { id: u[:id] } } })
end
# vuln-code-snippet end ruby_sqli_joins_string
