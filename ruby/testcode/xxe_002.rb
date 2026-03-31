require_relative 'shared'

module Nokogiri
  module XML
    def self.parse(input, url = nil, encoding = nil, options = nil)
      input
    end
  end
  def self.XML(input, url = nil, encoding = nil, options = nil)
    config = Object.new
    def config.nonet; self; end
    def config.noent; self; end
    yield config if block_given?
    input
  end
end

# vuln-code-snippet start ruby_xxe_nokogiri_nonet
def parse_xml_safe(req)
  xml_input = req.body_str
  doc = Nokogiri::XML(xml_input) { |config| config.nonet.noent } # vuln-code-snippet safe-line ruby_xxe_nokogiri_nonet
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_nokogiri_nonet
