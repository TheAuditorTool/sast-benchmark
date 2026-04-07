require_relative 'shared'

begin
  require 'mustache'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_mustache_fixed
def handler(req)
  output = Mustache.render('Hello {{name}}', { name: req.param('name') }) # vuln-code-snippet safe-line ruby_ssti_mustache_fixed
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_mustache_fixed
