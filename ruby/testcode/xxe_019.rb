require_relative 'shared'

module Nokogiri
  module XSLT
    def self.parse(xslt_str); Stylesheet.new; end
  end
  module XML
    def self.parse(input, *args); input; end
  end
  class Stylesheet
    def transform(doc); doc; end
  end
end

# vuln-code-snippet start ruby_xxe_xslt_transform
def transform_xml(req)
  xml_input = req.body_str
  xslt_input = req.param('xslt')
  stylesheet = Nokogiri::XSLT.parse(xslt_input)
  doc = Nokogiri::XML.parse(xml_input)
  result = stylesheet.transform(doc) # vuln-code-snippet vuln-line ruby_xxe_xslt_transform
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_xxe_xslt_transform
