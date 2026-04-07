require_relative 'shared'
require 'oga'

def parse_oga_no_external_handler(req)
  xml = req.post('xml')
  doc = Oga.parse_xml(xml)
  BenchmarkResponse.json({ root: doc.children.first&.name })
end
