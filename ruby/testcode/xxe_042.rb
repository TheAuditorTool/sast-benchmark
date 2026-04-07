require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_schema_validate_first
def parse_schema_validate_first_handler(req)
  xml = req.post('xml')
  xsd = Nokogiri::XML::Schema.new(File.read('schema.xsd'))
  doc = Nokogiri::XML.parse(xml) { |c| c.nonet.nodtdload } # vuln-code-snippet safe-line ruby_xxe_schema_validate_first
  errors = xsd.validate(doc)
  raise 'Invalid XML' unless errors.empty?
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_schema_validate_first
