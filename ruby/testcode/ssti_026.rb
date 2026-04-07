require_relative 'shared'

begin
  require 'mustache'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_mustache_lambda
def handler(req)
  output = Mustache.render(req.param('template'), { 'cmd' => -> { system('id') } }) # vuln-code-snippet vuln-line ruby_ssti_mustache_lambda
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_mustache_lambda
