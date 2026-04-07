require_relative 'shared'

def process_report(req)
  report_name = req.param('report')
  report_path = "/reports/#{report_name}"
  system("pdflatex #{report_path}")
  BenchmarkResponse.json({ status: 'processed' })
end
