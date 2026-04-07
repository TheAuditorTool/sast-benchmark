require_relative 'shared'

module Nokogiri
  module XML
    def self.parse(input, url = nil, encoding = nil, options = nil)
      input
    end
  end
  def self.XML(input, url = nil, encoding = nil, options = nil)
    config = Object.new
    def config.nonet; self; end
    def config.noent; self; end
    yield config if block_given?
    input
  end
end

def parse_xml_safe(req)
  xml_input = req.body_str
  doc = Nokogiri::XML(xml_input) { |config| config.nonet.noent }
  BenchmarkResponse.ok(doc.to_s)
end
