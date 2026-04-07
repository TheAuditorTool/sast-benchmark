require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_nokogiri_strict_flags2
def parse_strict_flags2_handler(req)
  xml = req.post('xml')
  opts = Nokogiri::XML::ParseOptions::STRICT | Nokogiri::XML::ParseOptions::NONET
  doc = Nokogiri::XML.parse(xml, nil, nil, opts) # vuln-code-snippet safe-line ruby_xxe_nokogiri_strict_flags2
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_nokogiri_strict_flags2
