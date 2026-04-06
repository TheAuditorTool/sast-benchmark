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

# vuln-code-snippet start ruby_xxe_sax_parser
def parse_sax(req)
  xml_input = req.body_str
  parser = Nokogiri::XML::SAX::Parser.new(DocHandler.new)
  parser.parse(xml_input) # vuln-code-snippet vuln-line ruby_xxe_sax_parser
  BenchmarkResponse.ok('parsed')
end
# vuln-code-snippet end ruby_xxe_sax_parser
