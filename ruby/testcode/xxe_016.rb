require_relative 'shared'

module Nokogiri; module XML
  module ParseOptions
    NONET = 2048
    STRICT = 1
  end
  def self.parse(input, url = nil, encoding = nil, options = nil); input; end
end; end

# vuln-code-snippet start ruby_xxe_nokogiri_strict_opts
def parse_xml_strict(req)
  xml_input = req.body_str
  opts = Nokogiri::XML::ParseOptions::STRICT | Nokogiri::XML::ParseOptions::NONET
  doc = Nokogiri::XML.parse(xml_input, nil, nil, opts) # vuln-code-snippet safe-line ruby_xxe_nokogiri_strict_opts
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_nokogiri_strict_opts
