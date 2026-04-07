require_relative 'shared'

def report_by_field(req)
  field = req.param('group_by')
  data = Report.group("#{field}").count
  BenchmarkResponse.json({ groups: data })
end
