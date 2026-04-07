require_relative 'shared'

REPORT_GEN_CMD = ['pdflatex', '/app/templates/report.tex', '-output-directory', '/app/output'].freeze

def generate_report(req)
  _format = req.param('format')
  system(*REPORT_GEN_CMD)
  BenchmarkResponse.json({ status: 'generated' })
end
