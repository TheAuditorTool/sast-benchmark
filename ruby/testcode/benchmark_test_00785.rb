require_relative 'shared'

module Nokogiri
  module XML
    class SAX
      class Parser
        def initialize(handler); @handler = handler; end
        def parse(input); input; end
      end
    end
  end
end

class DocHandler; end

def parse_sax(req)
  xml_input = req.body_str
  parser = Nokogiri::XML::SAX::Parser.new(DocHandler.new)
  parser.parse(xml_input)
  BenchmarkResponse.ok('parsed')
end
