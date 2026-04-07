require_relative 'shared'
require 'erb'

# Fixed ERB template constant — user input only fills escaped variable slots.
WELCOME_TPL = ERB.new('Welcome, <%= name %>! Your role is <%= role %>.').freeze

# vuln-code-snippet start ruby_ssti_no_template_from_user
def handler(req)
  output = WELCOME_TPL.result_with_hash({ # vuln-code-snippet safe-line ruby_ssti_no_template_from_user
    name: ERB::Util.html_escape(req.param('name')),
    role: ERB::Util.html_escape(req.param('role'))
  })
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_no_template_from_user
