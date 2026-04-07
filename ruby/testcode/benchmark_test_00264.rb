require_relative 'shared'

def handle_split_tainted(req)
  input = req.param('input')
  delim = req.param('delim')
  parts = input.split(Regexp.new(delim))
  BenchmarkResponse.json({ parts: parts })
end
