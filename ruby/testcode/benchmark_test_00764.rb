require_relative 'shared'
require 'rexml/document'
require 'rexml/security'

def parse_rexml_entity_limit_zero_handler(req)
  xml = req.post('xml')
  REXML::Security.entity_expansion_limit = 0
  doc = REXML::Document.new(xml)
  BenchmarkResponse.json({ root: doc.root&.name })
end
