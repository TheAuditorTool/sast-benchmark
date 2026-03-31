require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_erb_new
def ssti_erb_new(req)
  template = req.param('template')
  output = ERB.new(template).result # vuln-code-snippet vuln-line ruby_ssti_erb_new
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_erb_new
