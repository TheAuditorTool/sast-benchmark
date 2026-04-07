require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_escaped_vars_only
def handler(req)
  output = ERB.new('<p><%= name %></p>').result_with_hash({ name: ERB::Util.html_escape(req.param('name')) }) # vuln-code-snippet safe-line ruby_ssti_escaped_vars_only
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_escaped_vars_only
