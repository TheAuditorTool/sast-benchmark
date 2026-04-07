require_relative 'shared'
require 'ox'

def parse_ox_hash_handler(req)
  xml = req.post('xml')
  data = Ox.load(xml, mode: :hash)
  BenchmarkResponse.json({ data: data })
end
