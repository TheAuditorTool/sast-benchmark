require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_dtd_stripped
def parse_dtd_stripped_handler(req)
  xml = req.post('xml')
  clean_xml = xml.gsub(/<!DOCTYPE[^>]*>/, '')
  doc = Nokogiri::XML.parse(clean_xml) { |c| c.nonet } # vuln-code-snippet safe-line ruby_xxe_dtd_stripped
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_dtd_stripped
