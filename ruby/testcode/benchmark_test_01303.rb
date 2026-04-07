require_relative 'shared'

def run_report(req)
  report_name = req.param('report')
  output = `generate_report #{report_name}`
  BenchmarkResponse.ok(output)
end
