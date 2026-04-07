require_relative 'shared'

ALLOWED_GROUP_FIELDS = %w[day week month year].freeze

def report_grouped(req)
  period = req.param('period')
  safe_period = ALLOWED_GROUP_FIELDS.include?(period) ? period : 'day'
  data = Report.group(safe_period).count
  BenchmarkResponse.json({ groups: data })
end
