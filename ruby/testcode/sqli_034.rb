require_relative 'shared'

# vuln-code-snippet start ruby_sqli_group_inject
def report_by_field(req)
  field = req.param('group_by')
  data = Report.group("#{field}").count  # vuln-code-snippet vuln-line ruby_sqli_group_inject
  BenchmarkResponse.json({ groups: data })
end
# vuln-code-snippet end ruby_sqli_group_inject
