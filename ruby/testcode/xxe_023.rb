require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_nokogiri_default_opts
def parse_default_opts_handler(req)
  xml = req.post('xml')
  doc = Nokogiri::XML::Document.parse(xml, nil, nil, Nokogiri::XML::ParseOptions::DEFAULT_XML) # vuln-code-snippet vuln-line ruby_xxe_nokogiri_default_opts
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_nokogiri_default_opts
