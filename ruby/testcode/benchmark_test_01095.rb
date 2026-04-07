require_relative 'shared'
require 'nokogiri'

def parse_from_memory_handler(req)
  xml = req.post('xml')
  doc = Nokogiri::XML.from_memory(xml)
  BenchmarkResponse.json({ root: doc.root&.name })
end
