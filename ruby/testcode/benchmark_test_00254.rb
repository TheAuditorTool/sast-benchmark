require_relative 'shared'

module Ox
  def self.parse(input, opts = {}); input; end
end

def parse_ox(req)
  xml_input = req.body_str
  doc = Ox.parse(xml_input, mode: :generic)
  BenchmarkResponse.ok(doc.to_s)
end
