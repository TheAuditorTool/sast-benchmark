require_relative 'shared'

begin
  require 'prism'
rescue LoadError
end

def parse_ruby_source(req)
  source = req.param('code')
  parse_result = Prism.parse(source)
  BenchmarkResponse.json({ success: parse_result.success?, errors: parse_result.errors.map(&:message) })
end
