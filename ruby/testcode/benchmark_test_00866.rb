require_relative 'shared'
require 'nokogiri'

def parse_recover_dtd_handler(req)
  xml = req.post('xml')
  doc = Nokogiri::XML.parse(xml) { |c| c.recover.dtdload }
  BenchmarkResponse.json({ root: doc.root&.name })
end
