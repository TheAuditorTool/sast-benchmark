require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_nokogiri_nonet_nodtd
def parse_nonet_nodtd_handler(req)
  xml = req.post('xml')
  doc = Nokogiri::XML.parse(xml) { |c| c.nonet.nodtdload.noent } # vuln-code-snippet safe-line ruby_xxe_nokogiri_nonet_nodtd
  BenchmarkResponse.json({ root: doc.root&.name })
end
# vuln-code-snippet end ruby_xxe_nokogiri_nonet_nodtd
