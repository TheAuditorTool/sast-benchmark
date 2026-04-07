require_relative 'shared'

module Nokogiri; module XML
  module ParseOptions
    NONET = 2048
    DTDLOAD = 4
  end
  def self.parse(input, url = nil, encoding = nil, options = nil); input; end
end; end

def parse_xml_nonet(req)
  xml_input = req.body_str
  doc = Nokogiri::XML.parse(xml_input, nil, nil, Nokogiri::XML::ParseOptions::NONET)
  BenchmarkResponse.ok(doc.to_s)
end
