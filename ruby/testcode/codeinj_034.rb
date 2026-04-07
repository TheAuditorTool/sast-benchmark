require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_module_from_string
def load_extension(req)
  code = req.param('code')
  mod = Module.new do
    module_function
    eval(code) # vuln-code-snippet vuln-line ruby_codeinj_module_from_string
  end
  mod.extend(Object)
  BenchmarkResponse.json({ status: 'loaded' })
end
# vuln-code-snippet end ruby_codeinj_module_from_string
