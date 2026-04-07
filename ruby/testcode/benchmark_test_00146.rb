require_relative 'shared'

module Nokogiri; module XML
  module ParseOptions; NONET = 2048; end
  class Schema
    def self.new(schema_str, opts = nil); SchemaInstance.new; end
  end
  class SchemaInstance
    def validate(doc); []; end
  end
  def self.parse(input, url = nil, encoding = nil, options = nil); input; end
end; end

def parse_with_schema(req)
  xml_input = req.body_str
  schema = Nokogiri::XML::Schema.new(File.read('schema.xsd'))
  doc = Nokogiri::XML.parse(xml_input, nil, nil, Nokogiri::XML::ParseOptions::NONET)
  errors = schema.validate(doc)
  return BenchmarkResponse.bad_request('invalid xml') unless errors.empty?
  BenchmarkResponse.ok(doc.to_s)
end
