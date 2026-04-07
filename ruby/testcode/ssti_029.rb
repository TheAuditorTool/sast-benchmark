require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_dry_view_tpl
def handler(req)
  # User-controlled path determines which ERB file is loaded and executed
  output = ERB.new(File.read("templates/#{req.param('name')}.erb")).result # vuln-code-snippet vuln-line ruby_ssti_dry_view_tpl
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_dry_view_tpl
