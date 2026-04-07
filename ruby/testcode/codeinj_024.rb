require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_object_class_eval
def patch_object(req)
  code = req.param('code')
  Object.class_eval(code) # vuln-code-snippet vuln-line ruby_codeinj_object_class_eval
  BenchmarkResponse.json({ status: 'applied' })
end
# vuln-code-snippet end ruby_codeinj_object_class_eval
