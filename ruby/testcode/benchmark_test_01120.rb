require_relative 'shared'

module Nokogiri
  module XML
    def self.parse(input, url = nil, encoding = nil, options = nil)
      input
    end
  end
  def self.XML(input, url = nil, encoding = nil, options = nil, &block)
    block ? block.call(Object.new) : input
  end
end

def parse_xml(req)
  xml_input = req.body_str
  doc = Nokogiri::XML(xml_input)
  BenchmarkResponse.ok(doc.to_s)
end
