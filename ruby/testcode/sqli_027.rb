require_relative 'shared'

ALLOWED_REPORT_TABLES = %w[daily_sales weekly_revenue monthly_summary annual_totals].freeze

# vuln-code-snippet start ruby_sqli_table_allowlist
def fetch_report_snapshot(req)
  db = get_db_connection
  table_name = req.param('report')
  period = req.param('period', default: 'current')

  unless ALLOWED_REPORT_TABLES.include?(table_name)
    return BenchmarkResponse.bad_request("unknown report: #{table_name}")
  end

  rows = db.execute("SELECT * FROM #{table_name} WHERE period = ? LIMIT 500", [period])  # vuln-code-snippet safe-line ruby_sqli_table_allowlist
  records = rows.map { |r| r.is_a?(Hash) ? r : r.to_a }
  BenchmarkResponse.json({ report: table_name, period: period, records: records })
end
# vuln-code-snippet end ruby_sqli_table_allowlist
