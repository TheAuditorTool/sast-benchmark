require_relative 'shared'
require 'nokogiri'

def parse_strict_flags2_handler(req)
  xml = req.post('xml')
  opts = Nokogiri::XML::ParseOptions::STRICT | Nokogiri::XML::ParseOptions::NONET
  doc = Nokogiri::XML.parse(xml, nil, nil, opts)
  BenchmarkResponse.json({ root: doc.root&.name })
end
