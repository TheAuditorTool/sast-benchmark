require_relative 'shared'
require 'ox'

def parse_ox_tolerant_handler(req)
  xml = req.post('xml')
  doc = Ox.load(xml, effort: :tolerant)
  BenchmarkResponse.json({ root: doc.root.value })
end
