require_relative 'shared'
require 'ox'

# vuln-code-snippet start ruby_xxe_ox_permissive2
def parse_ox_permissive2_handler(req)
  xml = req.post('xml')
  doc = Ox.parse(xml) # vuln-code-snippet vuln-line ruby_xxe_ox_permissive2
  BenchmarkResponse.json({ root: doc.root.value })
end
# vuln-code-snippet end ruby_xxe_ox_permissive2
