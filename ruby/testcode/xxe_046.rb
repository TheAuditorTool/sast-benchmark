require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_html5_parser
def parse_html5_handler(req)
  input = req.post('data')
  doc = Nokogiri::HTML5.parse(input) # vuln-code-snippet safe-line ruby_xxe_html5_parser
  BenchmarkResponse.json({ title: doc.at('title')&.text })
end
# vuln-code-snippet end ruby_xxe_html5_parser
