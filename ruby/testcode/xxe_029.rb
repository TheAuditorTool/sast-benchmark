require_relative 'shared'
require 'rexml/document'
require 'rexml/xpath'

# vuln-code-snippet start ruby_xxe_rexml_xpath
def parse_rexml_xpath_handler(req)
  xml = req.post('xml')
  doc = REXML::Document.new(xml) # vuln-code-snippet vuln-line ruby_xxe_rexml_xpath
  user = REXML::XPath.first(doc, '//user')
  BenchmarkResponse.json({ user: user&.text })
end
# vuln-code-snippet end ruby_xxe_rexml_xpath
