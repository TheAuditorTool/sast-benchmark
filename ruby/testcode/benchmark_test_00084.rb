require_relative 'shared'
require 'nokogiri'

def parse_dtdload_handler(req)
  xml = req.post('xml')
  doc = Nokogiri::XML.parse(xml) { |c| c.dtdload }
  BenchmarkResponse.json({ root: doc.root&.name })
end
