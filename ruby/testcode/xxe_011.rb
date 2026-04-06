require_relative 'shared'

module REXML
  class Document
    def initialize(source = nil); @source = source; end
    def to_s; @source.to_s; end
  end
end

# vuln-code-snippet start ruby_xxe_rexml_parse
def parse_rexml(req)
  xml_input = req.body_str
  doc = REXML::Document.new(xml_input) # vuln-code-snippet vuln-line ruby_xxe_rexml_parse
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_rexml_parse
