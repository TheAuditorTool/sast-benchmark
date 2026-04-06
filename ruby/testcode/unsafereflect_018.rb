require_relative 'shared'

module Exporters
  class Base; def export; raise NotImplementedError; end; end
  class Csv < Base; def export; 'csv'; end; end
  class Pdf < Base; def export; 'pdf'; end; end
end

# vuln-code-snippet start ruby_reflect_ancestors_check
def export_checked(req)
  type = req.param('type')
  begin
    klass = Exporters.const_get(type.capitalize)
  rescue NameError
    return BenchmarkResponse.bad_request('unknown')
  end
  return BenchmarkResponse.bad_request('invalid') unless klass.ancestors.include?(Exporters::Base) # vuln-code-snippet safe-line ruby_reflect_ancestors_check
  BenchmarkResponse.ok(klass.new.export)
end
# vuln-code-snippet end ruby_reflect_ancestors_check
