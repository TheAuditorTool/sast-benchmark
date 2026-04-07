require_relative 'shared'
require 'oga'

# vuln-code-snippet start ruby_xxe_oga_xml
def parse_oga_xml_handler(req)
  xml = req.post('xml')
  doc = Oga.parse_xml(xml) # vuln-code-snippet vuln-line ruby_xxe_oga_xml
  BenchmarkResponse.json({ root: doc.children.first&.name })
end
# vuln-code-snippet end ruby_xxe_oga_xml
