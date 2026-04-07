require_relative 'shared'
require 'nokogiri'

def parse_default_opts_handler(req)
  xml = req.post('xml')
  doc = Nokogiri::XML::Document.parse(xml, nil, nil, Nokogiri::XML::ParseOptions::DEFAULT_XML)
  BenchmarkResponse.json({ root: doc.root&.name })
end
