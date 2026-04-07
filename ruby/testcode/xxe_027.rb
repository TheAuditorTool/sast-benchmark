require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_nokogiri_sax_default
def parse_nokogiri_sax_default_handler(req)
  xml = req.post('xml')
  class Handler < Nokogiri::XML::SAX::Document; end
  handler = Handler.new
  Nokogiri::XML::SAX::Parser.new(handler).parse(xml) # vuln-code-snippet vuln-line ruby_xxe_nokogiri_sax_default
  BenchmarkResponse.ok('parsed')
end
# vuln-code-snippet end ruby_xxe_nokogiri_sax_default
