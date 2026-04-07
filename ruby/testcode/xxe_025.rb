require_relative 'shared'
require 'rexml/document'

# vuln-code-snippet start ruby_xxe_rexml_full_doc
def parse_rexml_full_doc_handler(req)
  xml = req.post('xml')
  doc = REXML::Document.new(xml) # vuln-code-snippet vuln-line ruby_xxe_rexml_full_doc
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_rexml_full_doc
