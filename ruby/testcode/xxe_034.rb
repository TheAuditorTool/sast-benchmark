require_relative 'shared'
require 'rexml/document'
require 'rexml/sax2listener'
require 'rexml/parsers/sax2parser'

# vuln-code-snippet start ruby_xxe_sax2_rexml
def parse_sax2_rexml_handler(req)
  xml = req.post('xml')
  class MyListener; include REXML::SAX2Listener; end
  parser = REXML::SAX2Parser.new(xml) # vuln-code-snippet vuln-line ruby_xxe_sax2_rexml
  parser.parse
  BenchmarkResponse.ok('parsed')
end
# vuln-code-snippet end ruby_xxe_sax2_rexml
