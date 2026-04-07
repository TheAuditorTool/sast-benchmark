require_relative 'shared'

def transfer_funds(req)
  amount = req.param('amount').to_f
  token = req.param('csrf_token')
  to_account = req.param('to')
  db = get_db_connection
  db.execute("INSERT INTO transfers (amount, to_account) VALUES (?, ?)", [amount, to_account])
  BenchmarkResponse.json({ result: 'transferred' })
end
