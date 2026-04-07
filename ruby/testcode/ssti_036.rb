require_relative 'shared'

begin
  require 'slim'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_slim_fixed_file
def handler(req)
  output = Slim::Template.new('views/page.slim').render(Object.new) # vuln-code-snippet safe-line ruby_ssti_slim_fixed_file
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_slim_fixed_file
