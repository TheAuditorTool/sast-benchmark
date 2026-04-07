require_relative 'shared'
require 'nokogiri'

FIXED_XSD = File.read('schema.xsd') rescue '<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"/>'

# vuln-code-snippet start ruby_xxe_nokogiri_validator
def parse_nokogiri_validator_handler(req)
  xml = req.post('xml')
  xsd = Nokogiri::XML::Schema.new(FIXED_XSD)
  safe_opts = Nokogiri::XML::ParseOptions::NONET | Nokogiri::XML::ParseOptions::NODTDLOAD
  doc = Nokogiri::XML.parse(xml, nil, nil, safe_opts) # vuln-code-snippet safe-line ruby_xxe_nokogiri_validator
  errors = xsd.validate(doc)
  raise 'Invalid XML' unless errors.empty?
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_nokogiri_validator
