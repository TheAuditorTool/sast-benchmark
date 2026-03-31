require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_render_inline
def ssti_render_inline(req)
  expr = req.param('expr')
  erb_str = "<%= #{expr} %>"
  output = ERB.new(erb_str).result(binding) # vuln-code-snippet vuln-line ruby_ssti_render_inline
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_render_inline
