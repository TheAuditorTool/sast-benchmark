require_relative 'shared'

SAFE_MATH_PATTERN = /\A[\d\s\+\-\*\/\(\)\.]+\z/.freeze

# vuln-code-snippet start ruby_codeinj_safe_sandbox_vm
def sandboxed_eval(req)
  input = req.param('expr')
  unless input.match?(SAFE_MATH_PATTERN) # vuln-code-snippet safe-line ruby_codeinj_safe_sandbox_vm
    return BenchmarkResponse.bad_request('only numeric expressions allowed')
  end
  result = eval(input)
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_codeinj_safe_sandbox_vm
