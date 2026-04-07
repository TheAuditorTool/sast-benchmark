require_relative 'shared'

# vuln-code-snippet start ruby_sqli_arel_bind
def find_user_by_id_arel(req)
  raw_id = req.param('id')
  users = User.where(User.arel_table[:id].eq(raw_id.to_i))  # vuln-code-snippet safe-line ruby_sqli_arel_bind
  BenchmarkResponse.json({ user: users.first&.to_h })
end
# vuln-code-snippet end ruby_sqli_arel_bind
