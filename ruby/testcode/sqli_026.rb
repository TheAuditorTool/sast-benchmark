require_relative 'shared'

# vuln-code-snippet start ruby_sqli_table_interp
def export_report_data(req)
  db = get_db_connection
  table_name = req.param('report')
  report_date = req.param('date', default: Date.today.to_s)
  rows = db.execute("SELECT * FROM #{table_name} WHERE report_date = ? ORDER BY created_at DESC", [report_date])  # vuln-code-snippet vuln-line ruby_sqli_table_interp
  records = rows.map { |r| r.is_a?(Hash) ? r : r.to_a }
  BenchmarkResponse.json({ table: table_name, date: report_date, records: records })
end
# vuln-code-snippet end ruby_sqli_table_interp
