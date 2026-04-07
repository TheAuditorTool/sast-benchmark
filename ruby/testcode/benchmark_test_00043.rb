require_relative 'shared'
require 'libxml'

def parse_libxml_string_handler(req)
  xml = req.post('xml')
  doc = LibXML::XML::Parser.string(xml).parse
  BenchmarkResponse.json({ root: doc.root&.name })
end
