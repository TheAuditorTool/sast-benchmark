require_relative 'shared'

begin
  require 'tilt'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_tilt_file_only
def handler(req)
  output = Tilt.new('views/fixed.html.erb').render(Object.new, { name: req.param('name') }) # vuln-code-snippet safe-line ruby_ssti_tilt_file_only
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_tilt_file_only
