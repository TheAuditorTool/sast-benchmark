require_relative 'shared'

REPORT_GEN_CMD = ['pdflatex', '/app/templates/report.tex', '-output-directory', '/app/output'].freeze

# vuln-code-snippet start ruby_cmdi_frozen_constant_cmd
def generate_report(req)
  _format = req.param('format')  # tainted param present but never reaches command
  system(*REPORT_GEN_CMD)  # vuln-code-snippet safe-line ruby_cmdi_frozen_constant_cmd
  BenchmarkResponse.json({ status: 'generated' })
end
# vuln-code-snippet end ruby_cmdi_frozen_constant_cmd
