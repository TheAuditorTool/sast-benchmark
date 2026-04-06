require_relative 'shared'

module Nokogiri; module XML
  def self.parse(input, url = nil, encoding = nil, options = nil); input; end
  def self.new(input, url = nil, encoding = nil, options = nil); input; end
end; end

# vuln-code-snippet start ruby_xxe_nokogiri_default
def parse_xml_default(req)
  xml_input = req.body_str
  doc = Nokogiri::XML.parse(xml_input) # vuln-code-snippet vuln-line ruby_xxe_nokogiri_default
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_nokogiri_default
