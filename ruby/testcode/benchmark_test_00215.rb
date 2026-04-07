require_relative 'shared'

module Exporters; class Csv; def export; 'csv'; end; end; class Pdf; def export; 'pdf'; end; end; end

def export_data(req)
  type = req.param('type')
  klass = Object.const_get("Exporters::#{type}")
  BenchmarkResponse.ok(klass.new.export)
end
