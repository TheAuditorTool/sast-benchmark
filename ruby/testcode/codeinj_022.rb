require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_rubyvm_compile
def compile_and_run(req)
  code = req.param('code')
  iseq = RubyVM::InstructionSequence.compile(code)
  result = iseq.eval # vuln-code-snippet vuln-line ruby_codeinj_rubyvm_compile
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_codeinj_rubyvm_compile
