require_relative 'shared'

def match_with_timeout(req)
  text = req.param('text')
  previous_timeout = Regexp.timeout
  Regexp.timeout = 1.0
  begin
    matched = text.match(/(a+)+$/)
    BenchmarkResponse.ok(matched ? 'match' : 'no match')
  rescue Regexp::TimeoutError
    BenchmarkResponse.ok('timeout')
  ensure
    Regexp.timeout = previous_timeout
  end
end
