require_relative 'shared'

class XmlExporter
  def initialize(_data) = nil
  def to_s = 'XmlExporter'
end

class JsonExporter
  def initialize(_data) = nil
  def to_s = 'JsonExporter'
end

ALLOWED_EXPORTERS = [XmlExporter, JsonExporter].freeze

# vuln-code-snippet start ruby_reflect_const_fixed
def reflect_const_fixed(req)
  index = req.param('idx').to_i
  klass = ALLOWED_EXPORTERS[index] # vuln-code-snippet safe-line ruby_reflect_const_fixed
  return BenchmarkResponse.bad_request('invalid') unless klass
  BenchmarkResponse.ok(klass.new(req.param('data')).to_s)
end
# vuln-code-snippet end ruby_reflect_const_fixed
