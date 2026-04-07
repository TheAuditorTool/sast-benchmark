require_relative 'shared'
require 'rexml/document'
require 'rexml/xpath'

def parse_rexml_xpath_handler(req)
  xml = req.post('xml')
  doc = REXML::Document.new(xml)
  user = REXML::XPath.first(doc, '//user')
  BenchmarkResponse.json({ user: user&.text })
end
