require_relative 'shared'

SAFE_MATH_PATTERN = /\A[\d\s\+\-\*\/\(\)\.]+\z/.freeze

def sandboxed_eval(req)
  input = req.param('expr')
  unless input.match?(SAFE_MATH_PATTERN)
    return BenchmarkResponse.bad_request('only numeric expressions allowed')
  end
  result = eval(input)
  BenchmarkResponse.json({ result: result.to_s })
end
