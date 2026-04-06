require_relative 'shared'

module Nokogiri; module XML
  module ParseOptions
    NONET = 2048
    DTDLOAD = 4
  end
  def self.parse(input, url = nil, encoding = nil, options = nil); input; end
end; end

# vuln-code-snippet start ruby_xxe_nokogiri_nonet_flag
def parse_xml_nonet(req)
  xml_input = req.body_str
  doc = Nokogiri::XML.parse(xml_input, nil, nil, Nokogiri::XML::ParseOptions::NONET) # vuln-code-snippet safe-line ruby_xxe_nokogiri_nonet_flag
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_nokogiri_nonet_flag
