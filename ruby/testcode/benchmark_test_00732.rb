require_relative 'shared'

module WickedPdfClient
  def self.new; self; end
  def self.pdf_from_url(url); "pdf:#{url}"; end
end

REPORTS_BASE = 'https://reports.internal.example.com'.freeze

def generate_report_pdf(req)
  report_id = Integer(req.param('report_id'))
  WickedPdfClient.new.pdf_from_url("#{REPORTS_BASE}/#{report_id}")
  BenchmarkResponse.json({ ok: true })
end
