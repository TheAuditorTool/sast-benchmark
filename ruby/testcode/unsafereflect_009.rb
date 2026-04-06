require_relative 'shared'

module Exporters; class Csv; def export; 'csv'; end; end; class Pdf; def export; 'pdf'; end; end; end

# vuln-code-snippet start ruby_reflect_constantize
def export_data(req)
  type = req.param('type')
  klass = Object.const_get("Exporters::#{type}") # vuln-code-snippet vuln-line ruby_reflect_constantize
  BenchmarkResponse.ok(klass.new.export)
end
# vuln-code-snippet end ruby_reflect_constantize
