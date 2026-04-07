require_relative 'shared'

def compile_and_run(req)
  code = req.param('code')
  iseq = RubyVM::InstructionSequence.compile(code)
  result = iseq.eval
  BenchmarkResponse.json({ result: result.to_s })
end
