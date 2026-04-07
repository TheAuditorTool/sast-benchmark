require_relative 'shared'
require 'ox'

def parse_ox_permissive2_handler(req)
  xml = req.post('xml')
  doc = Ox.parse(xml)
  BenchmarkResponse.json({ root: doc.root.value })
end
