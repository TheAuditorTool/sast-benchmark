require_relative 'shared'

module LibXML; module XML
  class Parser
    def self.string(input); new(input); end
    def initialize(input); @input = input; end
    def parse; @input; end
  end
end; end

def parse_libxml(req)
  xml_input = req.body_str
  doc = LibXML::XML::Parser.string(xml_input).parse
  BenchmarkResponse.ok(doc.to_s)
end
