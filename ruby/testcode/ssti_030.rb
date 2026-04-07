require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_thor_template
def handler(req)
  # Thor-style template compilation from user-supplied string
  output = ERB.new(req.param('template_str')).result(binding) # vuln-code-snippet vuln-line ruby_ssti_thor_template
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_thor_template
