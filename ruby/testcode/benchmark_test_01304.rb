require_relative 'shared'

def run_report(req)
  report_name = req.param('report')
  output = ''
  IO.popen(['generate_report', report_name]) { |io| output = io.read }
  BenchmarkResponse.ok(output)
end
