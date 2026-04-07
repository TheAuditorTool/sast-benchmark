require_relative 'shared'

def safe_base; "base result"; end

# vuln-code-snippet start ruby_dynmethod_proc_curry_taint
def proc_curry_dispatch(req)
  method_name = req.param('method')
  result = method(method_name.to_sym).to_proc.curry.call # vuln-code-snippet vuln-line ruby_dynmethod_proc_curry_taint
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_dynmethod_proc_curry_taint
