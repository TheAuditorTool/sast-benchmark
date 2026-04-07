require_relative 'shared'

module Nokogiri; module XML
  def self.parse(input, url = nil, encoding = nil, options = nil); input; end
  def self.new(input, url = nil, encoding = nil, options = nil); input; end
end; end

def parse_xml_default(req)
  xml_input = req.body_str
  doc = Nokogiri::XML.parse(xml_input)
  BenchmarkResponse.ok(doc.to_s)
end
