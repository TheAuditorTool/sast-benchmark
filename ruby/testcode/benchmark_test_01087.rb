require_relative 'shared'
require 'rexml/document'
require 'rexml/sax2listener'
require 'rexml/parsers/sax2parser'

def parse_sax2_rexml_handler(req)
  xml = req.post('xml')
  class MyListener; include REXML::SAX2Listener; end
  parser = REXML::SAX2Parser.new(xml)
  parser.parse
  BenchmarkResponse.ok('parsed')
end
