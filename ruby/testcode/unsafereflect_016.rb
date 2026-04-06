require_relative 'shared'

module Exporters
  class Base; def export; raise NotImplementedError; end; end
  class Csv < Base; def export; 'csv'; end; end
  class Pdf < Base; def export; 'pdf'; end; end
end

# vuln-code-snippet start ruby_reflect_namespace_check
def export_namespaced(req)
  type = req.param('type')
  klass = Exporters.const_get(type.capitalize) # vuln-code-snippet safe-line ruby_reflect_namespace_check
  return BenchmarkResponse.bad_request('not an exporter') unless klass < Exporters::Base
  BenchmarkResponse.ok(klass.new.export)
end
# vuln-code-snippet end ruby_reflect_namespace_check
