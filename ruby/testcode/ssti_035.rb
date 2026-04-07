require_relative 'shared'

begin
  require 'haml'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_haml_fixed_file
def handler(req)
  output = Haml::Engine.new(File.read('templates/email.haml')).render(Object.new, name: req.param('name')) # vuln-code-snippet safe-line ruby_ssti_haml_fixed_file
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_haml_fixed_file
