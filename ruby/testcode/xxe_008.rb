require_relative 'shared'

module Nokogiri
  module XML
    STRICT = 1
    module ParseOptions
      STRICT = 1
    end
    def self.parse(input, url = nil, encoding = nil, options = nil)
      input
    end
  end
  def self.XML(input, url = nil, encoding = nil, options = nil)
    input
  end
end

# vuln-code-snippet start ruby_xxe_nokogiri_strict
def parse_xml_strict(req)
  xml_input = req.body_str
  doc = Nokogiri::XML(xml_input, nil, nil, Nokogiri::XML::ParseOptions::STRICT) # vuln-code-snippet safe-line ruby_xxe_nokogiri_strict
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_nokogiri_strict
