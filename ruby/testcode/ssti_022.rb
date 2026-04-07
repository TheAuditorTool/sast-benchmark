require_relative 'shared'

begin
  require 'erubi'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_erubi_capture
def handler(req)
  # Erubi::CaptureEndEngine variant — still compiles user input into executable Ruby
  src = Erubi::Engine.new(req.param('tpl'), escape: true).src # vuln-code-snippet vuln-line ruby_ssti_erubi_capture
  result = eval(src)
  BenchmarkResponse.html(result.to_s)
end
# vuln-code-snippet end ruby_ssti_erubi_capture
