require_relative 'shared'
require 'nokogiri'

# vuln-code-snippet start ruby_xxe_xslt_stylesheet
def parse_xslt_stylesheet_handler(req)
  xslt = Nokogiri::XSLT.parse(req.post('xslt')) # vuln-code-snippet vuln-line ruby_xxe_xslt_stylesheet
  doc = Nokogiri::XML('<root/>')
  result = xslt.transform(doc)
  BenchmarkResponse.html(result.to_s)
end
# vuln-code-snippet end ruby_xxe_xslt_stylesheet
