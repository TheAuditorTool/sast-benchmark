require_relative 'shared'

# vuln-code-snippet start ruby_csrf_token_in_url
def transfer_funds(req)
  amount = req.param('amount').to_f
  token = req.param('csrf_token')  # token in URL — leaks in logs/Referer
  to_account = req.param('to')
  db = get_db_connection
  db.execute("INSERT INTO transfers (amount, to_account) VALUES (?, ?)", [amount, to_account])  # vuln-code-snippet vuln-line ruby_csrf_token_in_url
  BenchmarkResponse.json({ result: 'transferred' })
end
# vuln-code-snippet end ruby_csrf_token_in_url
