require_relative 'shared'

def parse_request_body(req)
  require_relative('lib/parser')
  result = SimpleParser.parse(req.body_str)
  BenchmarkResponse.ok(result.to_s)
end
