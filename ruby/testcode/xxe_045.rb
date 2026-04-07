require_relative 'shared'
require 'rexml/document'
require 'rexml/security'

# vuln-code-snippet start ruby_xxe_rexml_sax_safe
def parse_rexml_sax_safe_handler(req)
  xml = req.post('xml')
  REXML::Security.entity_expansion_limit = 0
  parser = REXML::Document.new(xml) # vuln-code-snippet safe-line ruby_xxe_rexml_sax_safe
  BenchmarkResponse.json({ root: parser.root&.name })
end
# vuln-code-snippet end ruby_xxe_rexml_sax_safe
