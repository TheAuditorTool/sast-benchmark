require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_multihop_env
def process_report(req)
  report_name = req.param('report')
  # taint stored in local variable, passed to system call
  report_path = "/reports/#{report_name}"
  system("pdflatex #{report_path}")  # vuln-code-snippet vuln-line ruby_cmdi_multihop_env
  BenchmarkResponse.json({ status: 'processed' })
end
# vuln-code-snippet end ruby_cmdi_multihop_env
