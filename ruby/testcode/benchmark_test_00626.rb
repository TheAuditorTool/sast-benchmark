require_relative 'shared'

module REXML
  module Security
    def self.entity_expansion_limit; @limit || 10000; end
    def self.entity_expansion_limit=(val); @limit = val; end
  end
  class Document
    def initialize(source = nil); @source = source; end
    def to_s; @source.to_s; end
  end
end

def parse_rexml_limited(req)
  xml_input = req.body_str
  REXML::Security.entity_expansion_limit = 0
  doc = REXML::Document.new(xml_input)
  BenchmarkResponse.ok(doc.to_s)
end
