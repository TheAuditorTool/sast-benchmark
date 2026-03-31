require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_class_eval
def extend_class(req)
  code_string = req.param('definition')
  String.class_eval(code_string) # vuln-code-snippet vuln-line ruby_codeinj_class_eval
  BenchmarkResponse.ok("class extended")
end
# vuln-code-snippet end ruby_codeinj_class_eval
