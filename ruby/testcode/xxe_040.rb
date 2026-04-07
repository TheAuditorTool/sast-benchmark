require_relative 'shared'
require 'ox'

# vuln-code-snippet start ruby_xxe_ox_hash_mode
def parse_ox_hash_handler(req)
  xml = req.post('xml')
  data = Ox.load(xml, mode: :hash) # vuln-code-snippet safe-line ruby_xxe_ox_hash_mode
  BenchmarkResponse.json({ data: data })
end
# vuln-code-snippet end ruby_xxe_ox_hash_mode
