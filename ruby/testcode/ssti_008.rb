require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_escape_render
def ssti_escape_render(req)
  name  = escape_html(req.param('name'))
  score = escape_html(req.param('score'))
  template = ERB.new(File.read('templates/report.html.erb'))
  output = template.result(binding) # vuln-code-snippet safe-line ruby_ssti_escape_render
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_escape_render
