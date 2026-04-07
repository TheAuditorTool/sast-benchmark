require_relative 'shared'
require 'nokogiri'

FIXED_XSD = File.read('schema.xsd') rescue '<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"/>'

def parse_nokogiri_validator_handler(req)
  xml = req.post('xml')
  xsd = Nokogiri::XML::Schema.new(FIXED_XSD)
  safe_opts = Nokogiri::XML::ParseOptions::NONET | Nokogiri::XML::ParseOptions::NODTDLOAD
  doc = Nokogiri::XML.parse(xml, nil, nil, safe_opts)
  errors = xsd.validate(doc)
  raise 'Invalid XML' unless errors.empty?
  BenchmarkResponse.json({ root: doc.root&.name })
end
