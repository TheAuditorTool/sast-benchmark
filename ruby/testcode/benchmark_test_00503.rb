require_relative 'shared'

module WickedPdfClient
  def self.new; self; end
  def self.pdf_from_url(url); "pdf:#{url}"; end
end

def generate_pdf(req)
  pdf_url = req.param('url')
  WickedPdfClient.new.pdf_from_url(pdf_url)
  BenchmarkResponse.json({ ok: true })
end
