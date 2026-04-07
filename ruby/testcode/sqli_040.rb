require_relative 'shared'

ALLOWED_GROUP_FIELDS = %w[day week month year].freeze

# vuln-code-snippet start ruby_sqli_group_allowlist
def report_grouped(req)
  period = req.param('period')
  safe_period = ALLOWED_GROUP_FIELDS.include?(period) ? period : 'day'
  data = Report.group(safe_period).count  # vuln-code-snippet safe-line ruby_sqli_group_allowlist
  BenchmarkResponse.json({ groups: data })
end
# vuln-code-snippet end ruby_sqli_group_allowlist
