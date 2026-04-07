require_relative 'shared'
require 'ox'

# vuln-code-snippet start ruby_xxe_ox_tolerant
def parse_ox_tolerant_handler(req)
  xml = req.post('xml')
  doc = Ox.load(xml, effort: :tolerant) # vuln-code-snippet vuln-line ruby_xxe_ox_tolerant
  BenchmarkResponse.json({ root: doc.root.value })
end
# vuln-code-snippet end ruby_xxe_ox_tolerant
