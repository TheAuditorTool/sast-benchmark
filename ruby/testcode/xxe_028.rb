require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_nokogiri_from_memory
def parse_from_memory_handler(req)
  xml = req.post('xml')
  doc = Nokogiri::XML.from_memory(xml) # vuln-code-snippet vuln-line ruby_xxe_nokogiri_from_memory
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_nokogiri_from_memory
