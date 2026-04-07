require_relative 'shared'

# vuln-code-snippet start ruby_csrf_idempotent_key
def create_payment_idempotent(req)
  idempotency_key = req.header('Idempotency-Key')
  return BenchmarkResponse.bad_request('missing Idempotency-Key') unless idempotency_key&.length == 36
  db = get_db_connection
  existing = db.execute("SELECT id FROM payments WHERE idempotency_key = ?", [idempotency_key]).first
  return BenchmarkResponse.json({ result: existing[0] }) if existing
  amount = req.post('amount').to_f
  db.execute("INSERT INTO payments (amount, idempotency_key) VALUES (?, ?)", [amount, idempotency_key])  # vuln-code-snippet safe-line ruby_csrf_idempotent_key
  BenchmarkResponse.json({ result: 'created' })
end
# vuln-code-snippet end ruby_csrf_idempotent_key
