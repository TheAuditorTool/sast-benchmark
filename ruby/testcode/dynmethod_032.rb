require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_alias_dynamic
def alias_dynamic(req)
  klass = Class.new
  klass.send(:alias_method, req.param('new_name').to_sym, :to_s) # vuln-code-snippet vuln-line ruby_dynmethod_alias_dynamic
  BenchmarkResponse.json({ result: "aliased" })
end
# vuln-code-snippet end ruby_dynmethod_alias_dynamic
