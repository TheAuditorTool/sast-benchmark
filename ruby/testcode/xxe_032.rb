require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_nokogiri_html_xml
def parse_html_xml_handler(req)
  xml = req.post('data')
  doc = Nokogiri::HTML(xml) # vuln-code-snippet vuln-line ruby_xxe_nokogiri_html_xml
  BenchmarkResponse.json({ title: doc.at('title')&.text })
end
# vuln-code-snippet end ruby_xxe_nokogiri_html_xml
