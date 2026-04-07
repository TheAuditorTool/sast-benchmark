require_relative 'shared'
require 'rexml/document'
require 'rexml/security'

# vuln-code-snippet start ruby_xxe_rexml_entity_limit_zero
def parse_rexml_entity_limit_zero_handler(req)
  xml = req.post('xml')
  REXML::Security.entity_expansion_limit = 0
  doc = REXML::Document.new(xml) # vuln-code-snippet safe-line ruby_xxe_rexml_entity_limit_zero
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_rexml_entity_limit_zero
