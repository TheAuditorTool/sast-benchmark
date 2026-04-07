require_relative 'shared'
require 'rexml/document'
require 'rexml/security'

def parse_rexml_sax_safe_handler(req)
  xml = req.post('xml')
  REXML::Security.entity_expansion_limit = 0
  parser = REXML::Document.new(xml)
  BenchmarkResponse.json({ root: parser.root&.name })
end
