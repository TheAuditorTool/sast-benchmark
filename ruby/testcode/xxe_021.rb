require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_nokogiri_dtdload
def parse_dtdload_handler(req)
  xml = req.post('xml')
  doc = Nokogiri::XML.parse(xml) { |c| c.dtdload } # vuln-code-snippet vuln-line ruby_xxe_nokogiri_dtdload
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_nokogiri_dtdload
