require_relative 'shared'
require 'ox'

# vuln-code-snippet start ruby_xxe_ox_strict_mode
def parse_ox_strict_handler(req)
  xml = req.post('xml')
  doc = Ox.load(xml, mode: :strict, effort: :strict) # vuln-code-snippet safe-line ruby_xxe_ox_strict_mode
  BenchmarkResponse.json({ root: doc.root.value })
end
# vuln-code-snippet end ruby_xxe_ox_strict_mode
