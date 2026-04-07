require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_nokogiri_recover_dtd
def parse_recover_dtd_handler(req)
  xml = req.post('xml')
  doc = Nokogiri::XML.parse(xml) { |c| c.recover.dtdload } # vuln-code-snippet vuln-line ruby_xxe_nokogiri_recover_dtd
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_nokogiri_recover_dtd
