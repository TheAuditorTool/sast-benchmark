require_relative 'shared'
require 'ox'

def parse_ox_strict_handler(req)
  xml = req.post('xml')
  doc = Ox.load(xml, mode: :strict, effort: :strict)
  BenchmarkResponse.json({ root: doc.root.value })
end
