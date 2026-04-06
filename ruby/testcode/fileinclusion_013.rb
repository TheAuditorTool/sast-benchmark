require_relative 'shared'

# vuln-code-snippet start ruby_fi_binding_eval_file
def eval_template(req)
  template_name = req.param('template')
  binding.eval(File.read("templates/#{template_name}.rb")) # vuln-code-snippet vuln-line ruby_fi_binding_eval_file
  BenchmarkResponse.ok('rendered')
end
# vuln-code-snippet end ruby_fi_binding_eval_file
