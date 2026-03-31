require_relative 'shared'

module Nokogiri
  module XML
    class Document
      def self.parse(input, url = nil, encoding = nil, options = nil)
        new
      end
      def to_s; ''; end
    end
  end
end

# vuln-code-snippet start ruby_xxe_nokogiri_parse
def parse_xml_doc(req)
  xml_input = req.body_str
  doc = Nokogiri::XML::Document.parse(xml_input) # vuln-code-snippet vuln-line ruby_xxe_nokogiri_parse
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_nokogiri_parse
