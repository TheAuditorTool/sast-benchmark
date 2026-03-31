require_relative 'shared'

module REXML
  class Document
    def initialize(input)
      @input = input
    end
    def to_s; @input.to_s; end
  end
  module Security
    def self.entity_expansion_limit=(val); end
  end
end

# vuln-code-snippet start ruby_xxe_rexml
def parse_rexml(req)
  xml_input = req.body_str
  doc = REXML::Document.new(xml_input) # vuln-code-snippet vuln-line ruby_xxe_rexml
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_rexml
