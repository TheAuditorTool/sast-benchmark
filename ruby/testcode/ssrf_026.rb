require_relative 'shared'

module WickedPdfClient
  def self.new; self; end
  def self.pdf_from_url(url); "pdf:#{url}"; end
end

# vuln-code-snippet start ruby_ssrf_pdf_generator
def generate_pdf(req)
  pdf_url = req.param('url')
  WickedPdfClient.new.pdf_from_url(pdf_url) # vuln-code-snippet vuln-line ruby_ssrf_pdf_generator
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_pdf_generator
