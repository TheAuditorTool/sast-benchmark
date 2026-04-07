require_relative 'shared'

# vuln-code-snippet start ruby_sqli_joins_param
def users_in_role(req)
  role_id = req.param('role_id').to_i
  users = User.joins("INNER JOIN user_roles ON user_roles.user_id = users.id")
              .where("user_roles.role_id = ?", role_id)  # vuln-code-snippet safe-line ruby_sqli_joins_param
  BenchmarkResponse.json({ users: users.map { |u| { id: u[:id] } } })
end
# vuln-code-snippet end ruby_sqli_joins_param
