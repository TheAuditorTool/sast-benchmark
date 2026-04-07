require_relative 'shared'

def handler(req)
  user_data = req.post_data
  set_clauses = user_data.map { |k, v| "#{k} = '#{v}'" }.join(', ')
  sql = "UPDATE users SET #{set_clauses} WHERE id = 1"
  FakeActiveRecord::Base.find_by_sql(sql)
  BenchmarkResponse.ok('updated')
end
