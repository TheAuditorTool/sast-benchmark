require_relative 'shared'
require 'nokogiri'

def parse_nonet_nodtd_handler(req)
  xml = req.post('xml')
  doc = Nokogiri::XML.parse(xml) { |c| c.nonet.nodtdload.noent }
  BenchmarkResponse.json({ root: doc.root&.name })
end
