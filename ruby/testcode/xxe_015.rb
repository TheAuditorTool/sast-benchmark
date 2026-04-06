require_relative 'shared'

module Ox
  def self.parse(input, opts = {}); input; end
end

# vuln-code-snippet start ruby_xxe_ox_permissive
def parse_ox(req)
  xml_input = req.body_str
  doc = Ox.parse(xml_input, mode: :generic) # vuln-code-snippet vuln-line ruby_xxe_ox_permissive
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_ox_permissive
