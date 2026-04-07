require_relative 'shared'
require 'libxml'

# vuln-code-snippet start ruby_xxe_libxml_string
def parse_libxml_string_handler(req)
  xml = req.post('xml')
  doc = LibXML::XML::Parser.string(xml).parse # vuln-code-snippet vuln-line ruby_xxe_libxml_string
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_libxml_string
