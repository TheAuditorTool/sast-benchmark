require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_user_data_in_context_only
def handler(req)
  output = ERB.new('Hello <%= name %>').result_with_hash({ name: ERB::Util.html_escape(req.param('name')) }) # vuln-code-snippet safe-line ruby_ssti_user_data_in_context_only
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_user_data_in_context_only
