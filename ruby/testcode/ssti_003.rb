require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_erb_binding
def ssti_erb_binding(req)
  user_template = req.param('tpl')
  username = req.param('name')
  output = ERB.new(user_template).result(binding) # vuln-code-snippet vuln-line ruby_ssti_erb_binding
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_erb_binding
