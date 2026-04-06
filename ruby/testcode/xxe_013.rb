require_relative 'shared'

module LibXML; module XML
  class Parser
    def self.string(input); new(input); end
    def initialize(input); @input = input; end
    def parse; @input; end
  end
end; end

# vuln-code-snippet start ruby_xxe_libxml
def parse_libxml(req)
  xml_input = req.body_str
  doc = LibXML::XML::Parser.string(xml_input).parse # vuln-code-snippet vuln-line ruby_xxe_libxml
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_libxml
