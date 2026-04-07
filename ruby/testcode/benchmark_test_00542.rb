require_relative 'shared'
require 'nokogiri'

def parse_input_length_limit_handler(req)
  xml_data = req.post('xml')
  raise ArgumentError, 'too large' if xml_data.bytesize > 1_048_576
  doc = Nokogiri::XML.parse(xml_data) { |c| c.nonet.nodtdload }
  BenchmarkResponse.json({ root: doc.root&.name })
end
