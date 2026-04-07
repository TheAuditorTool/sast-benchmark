require_relative 'shared'
require 'oga'

# vuln-code-snippet start ruby_xxe_oga_no_external
def parse_oga_no_external_handler(req)
  xml = req.post('xml')
  doc = Oga.parse_xml(xml) # vuln-code-snippet safe-line ruby_xxe_oga_no_external # Oga does not fetch external entities by default
  BenchmarkResponse.json({ root: doc.children.first&.name })
end
# vuln-code-snippet end ruby_xxe_oga_no_external
