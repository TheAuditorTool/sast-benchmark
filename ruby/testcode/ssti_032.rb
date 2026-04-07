require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_ssti_erb_from_file_only
def handler(req)
  output = ERB.new(File.read('views/safe.html.erb')).result(binding) # vuln-code-snippet safe-line ruby_ssti_erb_from_file_only
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_erb_from_file_only
