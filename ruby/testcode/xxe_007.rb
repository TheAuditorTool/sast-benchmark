require_relative 'shared'

module Ox
  def self.load(xml, options = {})
    xml
  end
end

# vuln-code-snippet start ruby_xxe_ox_load
def parse_ox(req)
  xml_input = req.body_str
  doc = Ox.load(xml_input) # vuln-code-snippet vuln-line ruby_xxe_ox_load
  BenchmarkResponse.ok(doc.to_s)
end
# vuln-code-snippet end ruby_xxe_ox_load
