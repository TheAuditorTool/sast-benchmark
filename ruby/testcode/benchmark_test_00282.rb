require_relative 'shared'
require 'nokogiri'

def parse_options_safe_handler(req)
  xml = req.post('xml')
  opts = Nokogiri::XML::ParseOptions::NONET | Nokogiri::XML::ParseOptions::NODTDLOAD
  doc = Nokogiri::XML.parse(xml, nil, nil, opts)
  BenchmarkResponse.json({ root: doc.root&.name })
end
