require_relative 'shared'

# vuln-code-snippet start ruby_loginj_puts_debug
def debug_output(req)
  body = req.body_str
  puts "DEBUG: #{body}" # vuln-code-snippet vuln-line ruby_loginj_puts_debug
  BenchmarkResponse.ok('debug logged')
end
# vuln-code-snippet end ruby_loginj_puts_debug
