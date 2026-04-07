require_relative 'shared'

def debug_output(req)
  body = req.body_str
  puts "DEBUG: #{body}"
  BenchmarkResponse.ok('debug logged')
end
