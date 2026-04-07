require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_mail_template_inline
def handler(req)
  # Simulates ActionMailer inline template rendered with user-controlled body
  output = ERB.new(req.param('body')).result(binding) # vuln-code-snippet vuln-line ruby_ssti_mail_template_inline
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_mail_template_inline
