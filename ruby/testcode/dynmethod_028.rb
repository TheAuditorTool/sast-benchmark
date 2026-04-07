require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_instance_exec_taint
def instance_exec_dispatch(req)
  arg = req.param('arg')
  obj = Object.new
  obj.instance_exec(arg) { |a| send(a.to_sym) } # vuln-code-snippet vuln-line ruby_dynmethod_instance_exec_taint
  BenchmarkResponse.json({ result: "ok" })
end
# vuln-code-snippet end ruby_dynmethod_instance_exec_taint
