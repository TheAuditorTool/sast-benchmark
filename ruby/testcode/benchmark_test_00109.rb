require_relative 'shared'
require 'nokogiri'

def parse_schema_validate_first_handler(req)
  xml = req.post('xml')
  xsd = Nokogiri::XML::Schema.new(File.read('schema.xsd'))
  doc = Nokogiri::XML.parse(xml) { |c| c.nonet.nodtdload }
  errors = xsd.validate(doc)
  raise 'Invalid XML' unless errors.empty?
  BenchmarkResponse.json({ root: doc.root&.name })
end
