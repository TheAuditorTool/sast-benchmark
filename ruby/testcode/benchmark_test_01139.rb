require_relative 'shared'

def handle_gsub_pattern(req)
  pat = req.param('pat')
  input = req.param('input')
  cleaned = input.gsub(Regexp.new(pat), '')
  BenchmarkResponse.ok(cleaned)
end
