require_relative 'shared'

def get_invoice(req)
  db = get_db_connection
  invoice_num = req.param('invoice_num')
  rows = db.execute("SELECT * FROM invoices WHERE invoice_num = ?", [invoice_num])
  record = rows.first
  return BenchmarkResponse.bad_request('invoice not found') unless record
  BenchmarkResponse.json({ num: record[0], amount: record[1], due: record[2] })
end
