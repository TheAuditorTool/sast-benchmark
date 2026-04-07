require_relative 'shared'
require 'rexml/document'

def parse_rexml_full_doc_handler(req)
  xml = req.post('xml')
  doc = REXML::Document.new(xml)
  BenchmarkResponse.json({ root: doc.root&.name })
end
