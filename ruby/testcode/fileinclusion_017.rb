require_relative 'shared'

# vuln-code-snippet start ruby_fi_load_erb
def load_dynamic_erb(req)
  name = req.param('name')
  load("templates/#{name}.rb") # vuln-code-snippet vuln-line ruby_fi_load_erb
  BenchmarkResponse.ok('template loaded')
end
# vuln-code-snippet end ruby_fi_load_erb
