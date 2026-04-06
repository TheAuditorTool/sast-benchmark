require_relative 'shared'

# vuln-code-snippet start ruby_regex_timeout_global
def match_with_timeout(req)
  text = req.param('text')
  previous_timeout = Regexp.timeout
  Regexp.timeout = 1.0 # vuln-code-snippet safe-line ruby_regex_timeout_global
  begin
    matched = text.match(/(a+)+$/)
    BenchmarkResponse.ok(matched ? 'match' : 'no match')
  rescue Regexp::TimeoutError
    BenchmarkResponse.ok('timeout')
  ensure
    Regexp.timeout = previous_timeout
  end
end
# vuln-code-snippet end ruby_regex_timeout_global
