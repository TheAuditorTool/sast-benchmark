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

# vuln-code-snippet start ruby_xxe_rexml_limit
def parse_rexml_safe(req)
  xml_input = req.body_str
  REXML::Security.entity_expansion_limit = 0 # vuln-code-snippet safe-line ruby_xxe_rexml_limit
  doc = REXML::Document.new(xml_input)
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_rexml_limit
