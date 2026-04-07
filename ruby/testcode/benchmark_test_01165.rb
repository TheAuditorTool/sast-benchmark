require_relative 'shared'

def setup_parser(req)
  autoload(:Parser, File.join(__dir__, 'lib', 'parser'))
  BenchmarkResponse.ok("parser ready")
end
