require_relative 'shared'

def get_account_by_id(req)
  id = req.param('id')
  return BenchmarkResponse.bad_request('id required') if id.empty?

  condition = FakeActiveRecord::Base.sanitize_sql_array(["WHERE id = ?", id])
  sql = "SELECT id, name, email, plan FROM accounts #{condition}"
  rows = FakeActiveRecord::Base.find_by_sql(sql)
  account = rows.first
  return BenchmarkResponse.bad_request('not found') unless account

  BenchmarkResponse.json(account)
end
