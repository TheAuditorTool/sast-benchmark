require_relative 'shared'
require 'erb'

# Rails precompiled view template loaded at boot; handler only passes safe locals.
# The ERB template itself is a fixed compile-time constant — no user input reaches the template source.
PRECOMPILED_VIEW = ERB.new('<p>Hello <%= name %></p>').freeze

# vuln-code-snippet start ruby_ssti_precompiled_template
def handler(req)
  output = PRECOMPILED_VIEW.result_with_hash({ name: ERB::Util.html_escape(req.param('name')) }) # vuln-code-snippet safe-line ruby_ssti_precompiled_template
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_precompiled_template
