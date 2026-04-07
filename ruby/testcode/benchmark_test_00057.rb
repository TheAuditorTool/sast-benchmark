require_relative 'shared'

module REXML
  class Document
    def initialize(source = nil); @source = source; end
    def to_s; @source.to_s; end
  end
end

def parse_rexml(req)
  xml_input = req.body_str
  doc = REXML::Document.new(xml_input)
  BenchmarkResponse.ok(doc.to_s)
end
