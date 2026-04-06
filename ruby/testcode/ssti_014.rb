require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_erb_precompiled
def render_precompiled(req)
  name = escape_html(req.param('name'))
  template = ERB.new(File.read('templates/greeting.erb'))
  output = template.result_with_hash(name: name) # vuln-code-snippet safe-line ruby_ssti_erb_precompiled
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_erb_precompiled
