require_relative 'shared'

# vuln-code-snippet start ruby_fi_eval_file
def eval_plugin(req)
  path = req.param('path')
  eval(File.read(path)) # vuln-code-snippet vuln-line ruby_fi_eval_file
  BenchmarkResponse.ok('loaded')
end
# vuln-code-snippet end ruby_fi_eval_file
