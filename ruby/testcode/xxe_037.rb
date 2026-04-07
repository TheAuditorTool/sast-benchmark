require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_nokogiri_parse_options_safe
def parse_options_safe_handler(req)
  xml = req.post('xml')
  opts = Nokogiri::XML::ParseOptions::NONET | Nokogiri::XML::ParseOptions::NODTDLOAD
  doc = Nokogiri::XML.parse(xml, nil, nil, opts) # vuln-code-snippet safe-line ruby_xxe_nokogiri_parse_options_safe
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_nokogiri_parse_options_safe
