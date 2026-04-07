require_relative 'shared'

module Exporters
  class Base; def export; raise NotImplementedError; end; end
  class Csv < Base; def export; 'csv'; end; end
  class Pdf < Base; def export; 'pdf'; end; end
end

def export_namespaced(req)
  type = req.param('type')
  klass = Exporters.const_get(type.capitalize)
  return BenchmarkResponse.bad_request('not an exporter') unless klass < Exporters::Base
  BenchmarkResponse.ok(klass.new.export)
end
