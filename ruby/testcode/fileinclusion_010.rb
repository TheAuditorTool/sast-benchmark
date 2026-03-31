require_relative 'shared'

# vuln-code-snippet start ruby_fi_autoload_fixed
def setup_parser(req)
  autoload(:Parser, File.join(__dir__, 'lib', 'parser')) # vuln-code-snippet safe-line ruby_fi_autoload_fixed
  BenchmarkResponse.ok("parser ready")
end
# vuln-code-snippet end ruby_fi_autoload_fixed
