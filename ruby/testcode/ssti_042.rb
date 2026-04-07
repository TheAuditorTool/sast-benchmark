require_relative 'shared'

begin
  require 'erubi'
rescue LoadError
end

# Template is loaded from a fixed file path at boot, not from user input.
# vuln-code-snippet start ruby_ssti_erubi_precompiled2
def handler(req)
  src = Erubi::Engine.new(File.read('views/page.erb')).src # vuln-code-snippet safe-line ruby_ssti_erubi_precompiled2
  result = eval(src)
  BenchmarkResponse.html(result.to_s)
end
# vuln-code-snippet end ruby_ssti_erubi_precompiled2
