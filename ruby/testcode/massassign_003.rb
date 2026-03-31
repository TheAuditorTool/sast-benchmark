require_relative 'shared'

# vuln-code-snippet start ruby_massassign_update_all
def mass_assign_update_all(req)
  user_data = req.post_data # vuln-code-snippet vuln-line ruby_massassign_update_all
  set_clauses = user_data.map { |k, v| "#{k} = '#{v}'" }.join(', ')
  sql = "UPDATE users SET #{set_clauses} WHERE id = 1"
  FakeActiveRecord::Base.find_by_sql(sql)
  BenchmarkResponse.ok('updated')
end
# vuln-code-snippet end ruby_massassign_update_all
