require_relative 'shared'

def get_account_details(req)
  db = get_db_connection
  account_id = req.param('account_id')
  rows = db.execute("SELECT * FROM accounts WHERE id = ?", [account_id])
  record = rows.first
  return BenchmarkResponse.bad_request('not found') unless record
  BenchmarkResponse.json({ id: record[0], name: record[1], balance: record[2] })
end
