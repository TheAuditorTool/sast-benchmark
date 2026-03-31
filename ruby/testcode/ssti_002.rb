require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_file_template
def ssti_file_template(req)
  output = ERB.new(File.read('templates/page.html.erb')).result # vuln-code-snippet safe-line ruby_ssti_file_template
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_file_template
