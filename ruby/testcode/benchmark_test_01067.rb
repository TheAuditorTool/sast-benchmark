require_relative 'shared'
require 'shellwords'

def search_files(req)
  input = req.param('query')
  safe_input = Shellwords.escape(input)
  result = `grep -r
  BenchmarkResponse.ok(result)
end
