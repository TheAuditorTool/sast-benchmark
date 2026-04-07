require_relative 'shared'

def parse_sentence(req)
  input = req.param('input')
  matched = input =~ /(\w+\s+)*end/
  BenchmarkResponse.ok(matched ? 'found end' : 'no end')
end
