require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_action_view_inline
def handler(req)
  # Simulates ActionView render inline: with user-supplied template string
  inline_tpl = req.param('template')
  output = ERB.new(inline_tpl).result(binding) # vuln-code-snippet vuln-line ruby_ssti_action_view_inline
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_action_view_inline
