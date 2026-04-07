require_relative 'shared'

module Nokogiri; module XML
  module ParseOptions
    NONET = 2048
    STRICT = 1
  end
  def self.parse(input, url = nil, encoding = nil, options = nil); input; end
end; end

def parse_xml_strict(req)
  xml_input = req.body_str
  opts = Nokogiri::XML::ParseOptions::STRICT | Nokogiri::XML::ParseOptions::NONET
  doc = Nokogiri::XML.parse(xml_input, nil, nil, opts)
  BenchmarkResponse.ok(doc.to_s)
end
