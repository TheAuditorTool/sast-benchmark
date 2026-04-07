require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_input_length_limit
def parse_input_length_limit_handler(req)
  xml_data = req.post('xml')
  raise ArgumentError, 'too large' if xml_data.bytesize > 1_048_576
  doc = Nokogiri::XML.parse(xml_data) { |c| c.nonet.nodtdload } # vuln-code-snippet safe-line ruby_xxe_input_length_limit
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_input_length_limit
