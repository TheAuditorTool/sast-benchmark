require_relative 'shared'

module Ox
  def self.load(xml, options = {})
    xml
  end
end

def parse_ox(req)
  xml_input = req.body_str
  doc = Ox.load(xml_input)
  BenchmarkResponse.ok(doc.to_s)
end
