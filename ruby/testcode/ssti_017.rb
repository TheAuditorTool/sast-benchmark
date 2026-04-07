require_relative 'shared'

begin
  require 'erubi'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_erubi_engine
def handler(req)
  src = Erubi::Engine.new(req.param('tpl')).src # vuln-code-snippet vuln-line ruby_ssti_erubi_engine
  result = eval(src)
  BenchmarkResponse.html(result.to_s)
end
# vuln-code-snippet end ruby_ssti_erubi_engine
